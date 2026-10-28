// Copyright (C) 2026 Joncantplay
// SPDX-License-Identifier: GPL-3.0-or-later
//! One network stream per URL, shared by every radio tuned to it.
//!
//! A stream runs on its own thread: connect, resolve playlists, detect the codec,
//! decode, resample and hand the audio to all subscribed sources. If the connection
//! drops it reconnects with growing pauses until no source needs it anymore.

use std::cell::Cell;
use std::collections::VecDeque;
use std::io::{self, Cursor, Read};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::args::sqf_string;
use crate::callback;
use crate::decode::{self, PcmSink};
use crate::hls::HlsReader;
use crate::http::{self, decode_text, other_error, Abort, Url};
use crate::icy::IcyReader;
use crate::output;
use crate::playlist::{self, Playlist};
use crate::resample::{to_stereo, Resampler};
use crate::sniff::{self, Codec};

const SNIFF_BYTES: usize = 8 * 1024;
const MAX_PLAYLIST_BYTES: u64 = 512 * 1024;
const MAX_PLAYLIST_DEPTH: usize = 4;
const RETRY_DELAYS: [u64; 6] = [1, 2, 4, 8, 15, 30];
/// A connection that lasted this long counts as healthy and resets the retry delay.
const HEALTHY_AFTER: Duration = Duration::from_secs(30);
/// A connection that delivers data but no playable audio for this long counts as failed,
/// so the next fallback address gets its turn.
const NO_AUDIO_TIMEOUT: Duration = Duration::from_secs(20);

/// Seconds of audio buffered before a source starts playing.
const PRIME_SECONDS: f32 = 0.5;
/// Decoding pauses while a source has this much audio waiting. This paces HLS, which
/// delivers whole segments at once, and absorbs the initial burst of Icecast servers.
const AHEAD_SECONDS: f32 = 3.0;
/// Safety limit if a source stops consuming: old audio is dropped beyond this.
const MAX_BUFFER_SECONDS: f32 = 10.0;
const TRIM_TO_SECONDS: f32 = 3.0;

fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|e| e.into_inner())
}

/// Audio waiting to be mixed for one source (interleaved stereo at the device rate).
pub struct SourceBuffer {
    data: Mutex<VecDeque<f32>>,
    primed: AtomicBool,
}

impl SourceBuffer {
    pub fn new() -> Arc<SourceBuffer> {
        Arc::new(SourceBuffer { data: Mutex::new(VecDeque::new()), primed: AtomicBool::new(false) })
    }

    pub fn push(&self, samples: &[f32], rate: u32) {
        let mut data = lock(&self.data);
        data.extend(samples.iter().copied());
        let max = (rate as f32 * 2.0 * MAX_BUFFER_SECONDS) as usize;
        if data.len() > max {
            let keep = (rate as f32 * 2.0 * TRIM_TO_SECONDS) as usize & !1;
            let excess = data.len() - keep;
            data.drain(..excess);
        }
    }

    /// Seconds of audio waiting to be played.
    pub fn seconds(&self, rate: u32) -> f32 {
        lock(&self.data).len() as f32 / (rate.max(1) as f32 * 2.0)
    }

    /// Fills `out` with buffered audio and returns how many samples were written.
    /// Nothing is played until enough audio is buffered, so short gaps do not stutter.
    pub fn pop_into(&self, out: &mut [f32], rate: u32) -> usize {
        let mut data = lock(&self.data);
        if !self.primed.load(Ordering::Relaxed) {
            if data.len() < (rate as f32 * 2.0 * PRIME_SECONDS) as usize {
                return 0;
            }
            self.primed.store(true, Ordering::Relaxed);
        }
        let n = out.len().min(data.len()) & !1;
        for (o, s) in out.iter_mut().zip(data.drain(..n)) {
            *o = s;
        }
        if n < out.len() {
            self.primed.store(false, Ordering::Relaxed);
        }
        n
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum State {
    Connecting,
    Buffering,
    Playing,
    Reconnecting,
}

impl State {
    fn as_str(self) -> &'static str {
        match self {
            State::Connecting => "connecting",
            State::Buffering => "buffering",
            State::Playing => "playing",
            State::Reconnecting => "reconnecting",
        }
    }
}

struct Shared {
    url: String,
    stop: AtomicBool,
    abort: Arc<Abort>,
    subscribers: Mutex<Vec<(String, Arc<SourceBuffer>)>>,
    status: Mutex<(State, String)>,
    /// Current track title and a cover image URL announced by the station ("" if none).
    title: Mutex<(String, String)>,
    station: Mutex<String>,
}

impl Shared {
    fn send_status(id: &str, state: State, detail: &str) {
        callback::send("status", format!("[{},{},{}]", sqf_string(id), sqf_string(state.as_str()), sqf_string(detail)));
    }

    fn send_meta(id: &str, title: &str, station: &str, cover: &str) {
        callback::send(
            "meta",
            format!("[{},{},{},{}]", sqf_string(id), sqf_string(title), sqf_string(station), sqf_string(cover)),
        );
    }

    fn set_status(&self, state: State, detail: &str) {
        {
            let mut status = lock(&self.status);
            if status.0 == state && status.1 == detail {
                return;
            }
            *status = (state, detail.to_string());
        }
        for (id, _) in lock(&self.subscribers).iter() {
            Shared::send_status(id, state, detail);
        }
    }

    fn set_title(&self, title: String, cover: String) {
        *lock(&self.title) = (title.clone(), cover.clone());
        let station = lock(&self.station).clone();
        for (id, _) in lock(&self.subscribers).iter() {
            Shared::send_meta(id, &title, &station, &cover);
        }
    }

    fn stopped(&self) -> bool {
        self.stop.load(Ordering::SeqCst)
    }
}

pub struct Stream {
    shared: Arc<Shared>,
}

impl Stream {
    pub fn start(url: &str) -> Stream {
        let shared = Arc::new(Shared {
            url: url.to_string(),
            stop: AtomicBool::new(false),
            abort: Arc::new(Abort::new()),
            subscribers: Mutex::new(Vec::new()),
            status: Mutex::new((State::Connecting, String::new())),
            title: Mutex::new((String::new(), String::new())),
            station: Mutex::new(String::new()),
        });
        let worker = shared.clone();
        let spawned = std::thread::Builder::new().name("skywave-stream".into()).spawn(move || run(worker));
        if let Err(e) = spawned {
            callback::log(format!("could not start stream thread: {e}"));
        }
        Stream { shared }
    }

    pub fn subscribe(&self, id: &str, buffer: Arc<SourceBuffer>) {
        lock(&self.shared.subscribers).push((id.to_string(), buffer));
        let (state, detail) = lock(&self.shared.status).clone();
        Shared::send_status(id, state, &detail);
        let (title, cover) = lock(&self.shared.title).clone();
        if !title.is_empty() {
            Shared::send_meta(id, &title, &lock(&self.shared.station), &cover);
        }
    }

    /// Removes a subscriber. Returns true when nobody listens anymore.
    pub fn unsubscribe(&self, id: &str) -> bool {
        let mut subscribers = lock(&self.shared.subscribers);
        subscribers.retain(|(sid, _)| sid != id);
        subscribers.is_empty()
    }

    pub fn stop(&self) {
        self.shared.stop.store(true, Ordering::SeqCst);
        self.shared.abort.abort();
    }
}

/// The address list of a station: the main URL followed by fallbacks, separated by spaces.
fn addresses(list: &str) -> Vec<&str> {
    let urls: Vec<&str> = list.split_whitespace().collect();
    if urls.is_empty() {
        vec![list]
    } else {
        urls
    }
}

fn run(shared: Arc<Shared>) {
    let urls: Vec<String> = addresses(&shared.url).into_iter().map(str::to_string).collect();
    let mut index = 0;
    let mut attempt = 0usize;
    while !shared.stopped() {
        shared.set_status(if attempt == 0 { State::Connecting } else { State::Reconnecting }, "");
        let started = Instant::now();
        let url = &urls[index];
        let result = play_once(&shared, url);
        if shared.stopped() {
            break;
        }
        let reason = match result {
            Ok(()) => "stream ended".to_string(),
            Err(e) => e.to_string(),
        };
        callback::log(format!("{url}: {reason}"));
        if started.elapsed() >= HEALTHY_AFTER {
            // Lost a working connection: try the same address again first.
            attempt = 0;
        } else if urls.len() > 1 {
            index = (index + 1) % urls.len();
            if index != 0 {
                // Try the next fallback right away; pause only after a full round.
                continue;
            }
        }
        let delay = Duration::from_secs(RETRY_DELAYS[attempt.min(RETRY_DELAYS.len() - 1)]);
        attempt += 1;
        shared.set_status(State::Reconnecting, &reason);
        let until = Instant::now() + delay;
        while Instant::now() < until && !shared.stopped() {
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}

/// Reads up to `limit` bytes so the format can be inspected, and returns them
/// together with a reader that still yields the complete stream.
fn peek(mut reader: Box<dyn Read + Send>, limit: usize) -> io::Result<(Vec<u8>, Box<dyn Read + Send>)> {
    let mut head = Vec::with_capacity(limit);
    let mut chunk = [0u8; 4096];
    while head.len() < limit {
        match reader.read(&mut chunk) {
            Ok(0) => break,
            Ok(n) => head.extend_from_slice(&chunk[..n]),
            Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }
    let rest: Box<dyn Read + Send> = Box::new(Cursor::new(head.clone()).chain(reader));
    Ok((head, rest))
}

/// Connects and follows playlists until there is audio. Returns the content type,
/// the first bytes for format detection and a reader for the complete audio stream.
fn open_audio(shared: &Arc<Shared>, address: &str) -> io::Result<(String, Vec<u8>, Box<dyn Read + Send>)> {
    let mut url = Url::parse(address).map_err(other_error)?;
    for _ in 0..MAX_PLAYLIST_DEPTH {
        let response = http::get(&url, &shared.abort)?;
        if response.status != 200 {
            return Err(other_error(format!("server answered HTTP {}", response.status)));
        }
        if let Some(name) = response.header("icy-name") {
            *lock(&shared.station) = name.to_string();
        }
        let content_type = response.content_type();
        let interval = response.header("icy-metaint").and_then(|v| v.trim().parse::<usize>().ok()).unwrap_or(0);
        let body: Box<dyn Read + Send> = if interval > 0 {
            let target = shared.clone();
            Box::new(IcyReader::new(response.body, interval, Box::new(move |title, cover| target.set_title(title, cover))))
        } else {
            response.body
        };
        let (head, reader) = peek(body, SNIFF_BYTES)?;
        if !playlist::looks_like_playlist(&content_type, url.path_only(), &head) {
            return Ok((content_type, head, reader));
        }

        let mut raw = Vec::new();
        reader.take(MAX_PLAYLIST_BYTES).read_to_end(&mut raw)?;
        let text = decode_text(&raw);
        match playlist::parse(&text) {
            Playlist::Hls => {
                let target = shared.clone();
                let hls = HlsReader::open(&url, &text, shared.abort.clone(), Box::new(move |title| target.set_title(title, String::new())))?;
                let (head, reader) = peek(Box::new(hls), SNIFF_BYTES)?;
                return Ok((String::new(), head, reader));
            }
            Playlist::Entries(entries) => {
                let first = entries.first().ok_or_else(|| other_error("playlist is empty"))?;
                url = url.join(first).map_err(other_error)?;
            }
        }
    }
    Err(other_error("too many nested playlists"))
}

fn play_once(shared: &Arc<Shared>, address: &str) -> io::Result<()> {
    let (content_type, head, reader) = open_audio(shared, address)?;
    let codec = sniff::detect(&content_type, &head)
        .ok_or_else(|| other_error(format!("unsupported stream format '{content_type}'")))?;
    shared.set_status(State::Buffering, codec.name());
    let mut sink = Sink {
        shared,
        resampler: None,
        stereo: Vec::new(),
        resampled: Vec::new(),
        announced: false,
        last_audio: Instant::now(),
        starved: Cell::new(false),
    };
    let result = match codec {
        Codec::Aac => {
            let mut reader = reader;
            decode::run_aac(&mut reader, &mut sink)
        }
        Codec::Mp3 => decode::run_mp3(reader, &mut sink),
    };
    if sink.starved.get() {
        return Err(other_error(format!("no playable audio for {} s", NO_AUDIO_TIMEOUT.as_secs())));
    }
    result
}

struct Sink<'a> {
    shared: &'a Arc<Shared>,
    resampler: Option<Resampler>,
    stereo: Vec<[f32; 2]>,
    resampled: Vec<f32>,
    announced: bool,
    /// When the decoder last produced audio.
    last_audio: Instant,
    /// Set when the decoder went too long without audio; ends this connection.
    starved: Cell<bool>,
}

impl Sink<'_> {
    fn buffered_seconds(&self, rate: u32) -> f32 {
        lock(&self.shared.subscribers).iter().map(|(_, b)| b.seconds(rate)).fold(0.0, f32::max)
    }
}

impl PcmSink for Sink<'_> {
    fn push(&mut self, samples: &[f32], channels: usize, rate: u32) {
        let device_rate = output::device_rate();
        if self.resampler.as_ref().map(|r| r.rates()) != Some((rate, device_rate)) {
            self.resampler = Some(Resampler::new(rate, device_rate));
        }
        to_stereo(samples, channels, &mut self.stereo);
        self.resampled.clear();
        if let Some(resampler) = self.resampler.as_mut() {
            resampler.process(&self.stereo, &mut self.resampled);
        }
        for (_, buffer) in lock(&self.shared.subscribers).iter() {
            buffer.push(&self.resampled, device_rate);
        }
        if !self.announced {
            self.announced = true;
            self.shared.set_status(State::Playing, "");
        }
        // Wait while enough audio is queued, so decoding runs at playback speed. Waiting
        // here is not starving, so the clock restarts afterwards.
        while !self.shared.stopped() && self.buffered_seconds(device_rate) > AHEAD_SECONDS {
            std::thread::sleep(Duration::from_millis(20));
        }
        self.last_audio = Instant::now();
    }

    fn stopped(&self) -> bool {
        if self.shared.stopped() {
            return true;
        }
        if self.last_audio.elapsed() > NO_AUDIO_TIMEOUT {
            self.starved.set(true);
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_primes_before_playing() {
        let buffer = SourceBuffer::new();
        let mut out = vec![0.0; 100];
        buffer.push(&vec![0.5; 1000], 1000);
        assert_eq!(buffer.pop_into(&mut out, 1000), 100);
        let fresh = SourceBuffer::new();
        fresh.push(&vec![0.5; 500], 1000);
        assert_eq!(fresh.pop_into(&mut out, 1000), 0);
    }

    #[test]
    fn buffer_drops_old_audio_when_too_full() {
        let buffer = SourceBuffer::new();
        buffer.push(&vec![0.1; 21000], 1000);
        assert_eq!(lock(&buffer.data).len(), 6000);
        assert_eq!(buffer.seconds(1000), 3.0);
    }

    #[test]
    fn splits_address_lists() {
        assert_eq!(addresses("http://a/1 https://b/2"), vec!["http://a/1", "https://b/2"]);
        assert_eq!(addresses("http://a/1"), vec!["http://a/1"]);
    }

    #[test]
    fn peek_keeps_all_bytes() {
        let data: Vec<u8> = (0..20000u32).map(|i| i as u8).collect();
        let (head, mut rest) = peek(Box::new(Cursor::new(data.clone())), 8192).unwrap();
        assert_eq!(head.len(), 8192);
        let mut all = Vec::new();
        rest.read_to_end(&mut all).unwrap();
        assert_eq!(all, data);
    }
}
