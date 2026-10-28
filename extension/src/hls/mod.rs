// Copyright (C) 2026 Joncantplay
// SPDX-License-Identifier: GPL-3.0-or-later
//! HTTP Live Streaming. [`HlsReader`] turns an HLS stream into a continuous byte stream
//! of ADTS AAC or MP3 frames, so it plays through the same decoders as Icecast streams.
//!
//! Supported: master and media playlists, live and on-demand, segments as MPEG-TS,
//! packed audio (AAC/MP3 with ID3) or fragmented MP4, AES-128 encryption.

mod id3;
pub mod mp4;
pub mod playlist;
mod ts;

use std::collections::{HashMap, VecDeque};
use std::io::{self, Read};
use std::sync::Arc;
use std::time::{Duration, Instant};

use aes::cipher::block_padding::Pkcs7;
use aes::cipher::{BlockDecryptMut, KeyIvInit};

use crate::callback;
use crate::http::{self, decode_text, other_error, Abort, Url};
use crate::sniff::id3_length;
use mp4::{is_mp4, Mp4Demuxer};
use playlist::{choose_variant, extinf_title, Hls, MediaPlaylist, Segment};
use ts::{is_transport_stream, TsDemuxer};

const MAX_PLAYLIST_BYTES: u64 = 2 * 1024 * 1024;
const MAX_SEGMENT_BYTES: u64 = 32 * 1024 * 1024;
/// How many segments behind the live edge playback starts.
const LIVE_START_SEGMENTS: usize = 3;
const MAX_SEGMENT_FAILURES: u32 = 5;

type Aes128CbcDecryptor = cbc::Decryptor<aes::Aes128>;

fn fetch(url: &Url, abort: &Abort, limit: u64) -> io::Result<Vec<u8>> {
    let response = http::get(url, abort)?;
    if response.status != 200 {
        return Err(other_error(format!("HLS: server answered HTTP {} for {url}", response.status)));
    }
    let mut data = Vec::new();
    response.body.take(limit).read_to_end(&mut data)?;
    Ok(data)
}

fn decrypt(data: &mut Vec<u8>, key: &[u8; 16], iv: &[u8; 16]) -> io::Result<()> {
    let decryptor = Aes128CbcDecryptor::new_from_slices(key, iv).map_err(|_| other_error("HLS: invalid key"))?;
    let length = decryptor
        .decrypt_padded_mut::<Pkcs7>(data)
        .map_err(|_| other_error("HLS: segment could not be decrypted"))?
        .len();
    data.truncate(length);
    Ok(())
}

/// IV used when the playlist gives none: the media sequence number, big-endian.
fn sequence_iv(sequence: u64) -> [u8; 16] {
    let mut iv = [0u8; 16];
    iv[8..].copy_from_slice(&sequence.to_be_bytes());
    iv
}

pub struct HlsReader {
    playlist_url: Url,
    abort: Arc<Abort>,
    on_title: Box<dyn FnMut(String) + Send>,
    last_title: String,
    pending: VecDeque<Segment>,
    next_sequence: Option<u64>,
    target_duration: Duration,
    last_reload: Instant,
    last_new_segment: Instant,
    ended: bool,
    output: Vec<u8>,
    read_pos: usize,
    keys: HashMap<String, [u8; 16]>,
    current_map: Option<String>,
    ts: TsDemuxer,
    mp4: Mp4Demuxer,
}

impl HlsReader {
    /// Starts from an already downloaded playlist (master or media) found at `url`.
    pub fn open(url: &Url, text: &str, abort: Arc<Abort>, on_title: Box<dyn FnMut(String) + Send>) -> io::Result<HlsReader> {
        let (playlist_url, media) = match playlist::parse(text).map_err(other_error)? {
            Hls::Media(media) => (url.clone(), media),
            Hls::Master { variants, renditions } => {
                let uri = choose_variant(&variants, &renditions).ok_or_else(|| other_error("HLS: no playable variant"))?;
                let variant_url = url.join(&uri).map_err(other_error)?;
                let raw = fetch(&variant_url, &abort, MAX_PLAYLIST_BYTES)?;
                match playlist::parse(&decode_text(&raw)).map_err(other_error)? {
                    Hls::Media(media) => (variant_url, media),
                    Hls::Master { .. } => return Err(other_error("HLS: nested master playlists")),
                }
            }
        };
        let now = Instant::now();
        let mut reader = HlsReader {
            playlist_url,
            abort,
            on_title,
            last_title: String::new(),
            pending: VecDeque::new(),
            next_sequence: None,
            target_duration: Duration::from_secs(6),
            last_reload: now,
            last_new_segment: now,
            ended: false,
            output: Vec::new(),
            read_pos: 0,
            keys: HashMap::new(),
            current_map: None,
            ts: TsDemuxer::new(),
            mp4: Mp4Demuxer::new(),
        };
        reader.apply(media);
        if reader.pending.is_empty() && reader.ended {
            return Err(other_error("HLS: playlist has no segments"));
        }
        Ok(reader)
    }

    /// Queues the segments of a (re)loaded media playlist that were not queued yet.
    fn apply(&mut self, media: MediaPlaylist) {
        self.target_duration = Duration::from_secs_f32(media.target_duration.clamp(1.0, 30.0));
        self.ended = media.ended;
        let Some(first) = media.segments.first().map(|s| s.sequence) else { return };
        let next = match self.next_sequence {
            None if media.ended => first,
            None => {
                let start = media.segments.len().saturating_sub(LIVE_START_SEGMENTS);
                media.segments[start].sequence
            }
            // Fell behind the playlist window: continue at its start.
            Some(next) if next < first => first,
            Some(next) => next,
        };
        let mut added = false;
        for segment in media.segments.into_iter().filter(|s| s.sequence >= next) {
            self.next_sequence = Some(segment.sequence + 1);
            self.pending.push_back(segment);
            added = true;
        }
        if self.next_sequence.is_none() {
            self.next_sequence = Some(next);
        }
        if added {
            self.last_new_segment = Instant::now();
        }
    }

    fn wait_until(&self, deadline: Instant) -> io::Result<()> {
        while Instant::now() < deadline {
            if self.abort.is_aborted() {
                return Err(other_error("stopped"));
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        Ok(())
    }

    fn reload(&mut self) -> io::Result<()> {
        // Reload once per target duration; sooner after a reload that brought nothing new.
        let wait = if self.last_new_segment > self.last_reload { self.target_duration } else { self.target_duration / 2 };
        self.wait_until(self.last_reload + wait)?;
        let raw = fetch(&self.playlist_url, &self.abort, MAX_PLAYLIST_BYTES)?;
        self.last_reload = Instant::now();
        match playlist::parse(&decode_text(&raw)).map_err(other_error)? {
            Hls::Media(media) => self.apply(media),
            Hls::Master { .. } => return Err(other_error("HLS: media playlist turned into a master playlist")),
        }
        if self.pending.is_empty() && self.last_new_segment.elapsed() > self.target_duration * 4 {
            return Err(other_error("HLS: playlist stopped updating"));
        }
        Ok(())
    }

    fn key(&mut self, uri: &str) -> io::Result<[u8; 16]> {
        if let Some(key) = self.keys.get(uri) {
            return Ok(*key);
        }
        let url = self.playlist_url.join(uri).map_err(other_error)?;
        let raw = fetch(&url, &self.abort, 1024)?;
        let key: [u8; 16] = raw.as_slice().try_into().map_err(|_| other_error("HLS: key is not 16 bytes"))?;
        if self.keys.len() > 16 {
            self.keys.clear();
        }
        self.keys.insert(uri.to_string(), key);
        Ok(key)
    }

    fn emit_title(&mut self, title: String) {
        if !title.is_empty() && title != self.last_title {
            self.last_title = title.clone();
            (self.on_title)(title);
        }
    }

    fn load_segment(&mut self, segment: &Segment) -> io::Result<()> {
        if segment.map != self.current_map {
            if let Some(map) = &segment.map {
                let url = self.playlist_url.join(map).map_err(other_error)?;
                let init = fetch(&url, &self.abort, MAX_SEGMENT_BYTES)?;
                self.mp4.parse_init(&init).map_err(other_error)?;
            }
            self.current_map = segment.map.clone();
        }

        let url = self.playlist_url.join(&segment.uri).map_err(other_error)?;
        let mut data = fetch(&url, &self.abort, MAX_SEGMENT_BYTES)?;
        if let Some(key) = &segment.key {
            if key.method != "AES-128" {
                return Err(other_error(format!("HLS: encryption {} is not supported", key.method)));
            }
            let iv = key.iv.unwrap_or_else(|| sequence_iv(segment.sequence));
            let secret = self.key(&key.uri.clone())?;
            decrypt(&mut data, &secret, &iv)?;
        }
        if let Some(title) = extinf_title(&segment.title) {
            self.emit_title(title);
        }

        if is_transport_stream(&data) {
            self.ts.push(&data, &mut self.output);
            if let Some(title) = self.ts.take_title() {
                self.emit_title(title);
            }
        } else if is_mp4(&data) || segment.map.is_some() {
            self.mp4.push(&data, &mut self.output).map_err(other_error)?;
        } else {
            // Packed audio: ADTS or MP3 frames, usually behind an ID3 tag with timing info.
            let tag = id3_length(&data).min(data.len());
            if let Some(title) = id3::title(&data[..tag]) {
                self.emit_title(title);
            }
            self.output.extend_from_slice(&data[tag..]);
        }
        Ok(())
    }

    /// Loads segments until there is new audio. Returns false at the end of a finished stream.
    fn refill(&mut self) -> io::Result<bool> {
        let mut failures = 0;
        loop {
            if self.abort.is_aborted() {
                return Err(other_error("stopped"));
            }
            if let Some(segment) = self.pending.pop_front() {
                match self.load_segment(&segment) {
                    Ok(()) if self.output.len() > self.read_pos => return Ok(true),
                    Ok(()) => continue,
                    Err(e) => {
                        failures += 1;
                        callback::log(format!("HLS segment {} failed: {e}", segment.sequence));
                        if failures >= MAX_SEGMENT_FAILURES {
                            return Err(e);
                        }
                        continue;
                    }
                }
            }
            if self.ended {
                return Ok(false);
            }
            self.reload()?;
        }
    }
}

impl Read for HlsReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        if self.read_pos >= self.output.len() {
            self.output.clear();
            self.read_pos = 0;
            if !self.refill()? {
                return Ok(0);
            }
        }
        let n = buf.len().min(self.output.len() - self.read_pos);
        buf[..n].copy_from_slice(&self.output[self.read_pos..self.read_pos + n]);
        self.read_pos += n;
        Ok(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aes::cipher::BlockEncryptMut;

    #[test]
    fn decrypts_aes_128_segments() {
        let key = [7u8; 16];
        let iv = sequence_iv(42);
        let plain = b"ADTS frames and more audio data".to_vec();
        let mut buffer = vec![0u8; plain.len() + 16];
        buffer[..plain.len()].copy_from_slice(&plain);
        let encrypted = cbc::Encryptor::<aes::Aes128>::new_from_slices(&key, &iv)
            .unwrap()
            .encrypt_padded_mut::<Pkcs7>(&mut buffer, plain.len())
            .unwrap()
            .to_vec();
        let mut data = encrypted;
        decrypt(&mut data, &key, &iv).unwrap();
        assert_eq!(data, plain);
        assert_eq!(iv[15], 42);
    }

    fn media(first: u64, count: u64, ended: bool) -> MediaPlaylist {
        let segments = (first..first + count)
            .map(|sequence| Segment { uri: format!("s{sequence}.aac"), sequence, key: None, map: None, title: String::new() })
            .collect();
        MediaPlaylist { target_duration: 6.0, segments, ended }
    }

    fn reader() -> HlsReader {
        let now = Instant::now();
        HlsReader {
            playlist_url: Url::parse("http://example.com/live.m3u8").unwrap(),
            abort: Arc::new(Abort::new()),
            on_title: Box::new(|_| {}),
            last_title: String::new(),
            pending: VecDeque::new(),
            next_sequence: None,
            target_duration: Duration::from_secs(6),
            last_reload: now,
            last_new_segment: now,
            ended: false,
            output: Vec::new(),
            read_pos: 0,
            keys: HashMap::new(),
            current_map: None,
            ts: TsDemuxer::new(),
            mp4: Mp4Demuxer::new(),
        }
    }

    #[test]
    fn live_playback_starts_near_the_edge_and_follows_updates() {
        let mut r = reader();
        r.apply(media(10, 6, false));
        let queued: Vec<u64> = r.pending.iter().map(|s| s.sequence).collect();
        assert_eq!(queued, vec![13, 14, 15]);
        r.pending.clear();
        r.apply(media(12, 6, false));
        let queued: Vec<u64> = r.pending.iter().map(|s| s.sequence).collect();
        assert_eq!(queued, vec![16, 17]);
    }

    #[test]
    fn on_demand_playback_starts_at_the_beginning() {
        let mut r = reader();
        r.apply(media(0, 4, true));
        assert_eq!(r.pending.len(), 4);
        assert!(r.ended);
    }

    #[test]
    fn jumps_ahead_when_falling_behind() {
        let mut r = reader();
        r.apply(media(10, 3, false));
        r.pending.clear();
        r.apply(media(50, 3, false));
        assert_eq!(r.pending.front().map(|s| s.sequence), Some(50));
    }
}
