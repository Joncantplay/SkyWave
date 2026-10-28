// Copyright (C) 2026 Joncantplay
// SPDX-License-Identifier: GPL-3.0-or-later
//! Decoding loops: read compressed audio from a stream and hand PCM to a sink.

use std::io::{self, Read};
use std::sync::Mutex;

use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::{MediaSourceStream, ReadOnlySource};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use crate::faad::AacDecoder;
use crate::http::other_error;
use crate::sniff::{adts_frame_length, find_sync};

pub trait PcmSink {
    /// Receives interleaved samples in the range -1..1.
    fn push(&mut self, samples: &[f32], channels: usize, rate: u32);
    /// True when decoding should end.
    fn stopped(&self) -> bool;
}

const AAC_READ_AHEAD: usize = 8 * 1024;
const AAC_MAX_ERRORS: u32 = 50;

/// Reads from `reader` until `buffer[start..]` holds at least `want` bytes or the stream ends.
fn fill(reader: &mut dyn Read, buffer: &mut Vec<u8>, start: usize, want: usize, eof: &mut bool) -> io::Result<()> {
    let mut chunk = [0u8; 16 * 1024];
    while !*eof && buffer.len() - start < want {
        match reader.read(&mut chunk) {
            Ok(0) => *eof = true,
            Ok(n) => buffer.extend_from_slice(&chunk[..n]),
            Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

/// Decodes AAC in ADTS framing (AAC-LC, HE-AAC v1 and v2) with FAAD2.
pub fn run_aac(reader: &mut dyn Read, sink: &mut dyn PcmSink) -> io::Result<()> {
    let mut decoder = AacDecoder::new().ok_or_else(|| other_error("AAC decoder unavailable"))?;
    let mut buffer: Vec<u8> = Vec::with_capacity(64 * 1024);
    let mut start = 0;
    let mut eof = false;

    // Find the first frame; live streams may start in the middle of one.
    loop {
        fill(reader, &mut buffer, start, 16 * 1024, &mut eof)?;
        if let Some(offset) = find_sync(&buffer[start..], adts_frame_length) {
            start += offset;
            break;
        }
        if eof || buffer.len() > 256 * 1024 {
            return Err(other_error("no AAC frames found in stream"));
        }
        start = buffer.len().saturating_sub(8);
    }
    fill(reader, &mut buffer, start, AAC_READ_AHEAD, &mut eof)?;
    let skip = decoder.init_adts(&mut buffer[start..]).map_err(other_error)?;
    start += skip;

    let mut errors = 0u32;
    loop {
        if sink.stopped() {
            return Ok(());
        }
        fill(reader, &mut buffer, start, AAC_READ_AHEAD, &mut eof)?;
        if buffer.len() - start < 7 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "stream ended"));
        }

        let frame = decoder.decode(&mut buffer[start..]);
        let consumed = frame.consumed;
        if let Some(message) = frame.error {
            errors += 1;
            if errors > AAC_MAX_ERRORS {
                return Err(other_error(format!("AAC decoding failed: {message}")));
            }
            // Skip the damaged frame and look for the next header.
            let from = start + consumed.max(1);
            start = match find_sync(&buffer[from..], adts_frame_length) {
                Some(offset) => from + offset,
                None => buffer.len(),
            };
        } else {
            errors = errors.saturating_sub(1);
            if !frame.samples.is_empty() && frame.channels > 0 && frame.rate > 0 {
                sink.push(frame.samples, frame.channels, frame.rate);
            }
            start += consumed.max(if frame.samples.is_empty() { 1 } else { 0 });
        }

        if start > 32 * 1024 {
            buffer.drain(..start);
            start = 0;
        }
    }
}

/// Makes a plain reader usable as a Symphonia media source (which must be `Sync`).
struct SyncReader(Mutex<Box<dyn Read + Send>>);

impl Read for SyncReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.get_mut().unwrap_or_else(|e| e.into_inner()).read(buf)
    }
}

fn symphonia_error(error: SymphoniaError) -> io::Error {
    match error {
        SymphoniaError::IoError(e) => e,
        other => other_error(format!("MP3 decoding failed: {other}")),
    }
}

/// Decodes MPEG audio (MP3) with Symphonia.
pub fn run_mp3(reader: Box<dyn Read + Send>, sink: &mut dyn PcmSink) -> io::Result<()> {
    let source = ReadOnlySource::new(SyncReader(Mutex::new(reader)));
    let stream = MediaSourceStream::new(Box::new(source), Default::default());
    let mut hint = Hint::new();
    hint.with_extension("mp3").mime_type("audio/mpeg");
    let probed = symphonia::default::get_probe()
        .format(&hint, stream, &FormatOptions::default(), &MetadataOptions::default())
        .map_err(symphonia_error)?;
    let mut format = probed.format;
    let track = format.default_track().ok_or_else(|| other_error("no audio track in stream"))?;
    let track_id = track.id;
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .map_err(symphonia_error)?;

    loop {
        if sink.stopped() {
            return Ok(());
        }
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(SymphoniaError::ResetRequired) => {
                decoder.reset();
                continue;
            }
            Err(e) => return Err(symphonia_error(e)),
        };
        if packet.track_id() != track_id {
            continue;
        }
        match decoder.decode(&packet) {
            Ok(audio) => {
                let spec = *audio.spec();
                let channels = spec.channels.count();
                if channels == 0 || audio.frames() == 0 {
                    continue;
                }
                let mut samples = SampleBuffer::<f32>::new(audio.capacity() as u64, spec);
                samples.copy_interleaved_ref(audio);
                sink.push(samples.samples(), channels, spec.rate);
            }
            Err(SymphoniaError::DecodeError(_)) => continue,
            Err(e) => return Err(symphonia_error(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct CountingSink {
        pushes: usize,
    }

    impl PcmSink for CountingSink {
        fn push(&mut self, _: &[f32], _: usize, _: u32) {
            self.pushes += 1;
        }
        fn stopped(&self) -> bool {
            false
        }
    }

    #[test]
    fn aac_without_frames_is_an_error() {
        let data = vec![0u8; 4096];
        let mut sink = CountingSink { pushes: 0 };
        let result = run_aac(&mut &data[..], &mut sink);
        assert!(result.is_err());
        assert_eq!(sink.pushes, 0);
    }

    #[test]
    fn aac_decoder_opens() {
        assert!(AacDecoder::new().is_some());
    }
}
