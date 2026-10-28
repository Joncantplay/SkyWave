// Copyright (C) 2026 Joncantplay
// SPDX-License-Identifier: GPL-3.0-or-later
//! Fragmented MP4 (CMAF) demuxer for HLS audio. AAC frames are rewrapped in ADTS
//! headers so the same decoder path handles every HLS format.

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Track {
    /// ADTS profile (object type - 1), sampling frequency index, channel configuration.
    Aac { profile: u8, frequency_index: u8, channels: u8 },
    Mp3,
}

pub struct Mp4Demuxer {
    track: Option<Track>,
}

/// Returns true if `data` starts with an MP4 box that segments begin with.
pub fn is_mp4(data: &[u8]) -> bool {
    data.len() >= 8 && matches!(&data[4..8], b"ftyp" | b"styp" | b"moof" | b"sidx" | b"moov" | b"emsg" | b"prft")
}

struct BoxInfo<'a> {
    kind: [u8; 4],
    start: usize,
    payload: &'a [u8],
    payload_start: usize,
}

/// Iterates the boxes directly inside `data`.
fn boxes(data: &[u8]) -> Vec<BoxInfo<'_>> {
    let mut list = Vec::new();
    let mut pos = 0;
    while pos + 8 <= data.len() {
        let size32 = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
        let kind = [data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7]];
        let (size, header) = match size32 {
            0 => (data.len() - pos, 8),
            1 => {
                if pos + 16 > data.len() {
                    break;
                }
                let mut b = [0u8; 8];
                b.copy_from_slice(&data[pos + 8..pos + 16]);
                (u64::from_be_bytes(b) as usize, 16)
            }
            n => (n, 8),
        };
        if size < header || pos + size > data.len() {
            break;
        }
        list.push(BoxInfo { kind, start: pos, payload: &data[pos + header..pos + size], payload_start: pos + header });
        pos += size;
    }
    list
}

fn child<'a>(data: &'a [u8], kind: &[u8; 4]) -> Option<&'a [u8]> {
    boxes(data).into_iter().find(|b| &b.kind == kind).map(|b| b.payload)
}

fn path<'a>(data: &'a [u8], kinds: &[&[u8; 4]]) -> Option<&'a [u8]> {
    kinds.iter().try_fold(data, |current, kind| child(current, kind))
}

fn be_u32(d: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_be_bytes(d.get(at..at + 4)?.try_into().ok()?))
}

struct Bits<'a> {
    data: &'a [u8],
    pos: usize,
}

impl Bits<'_> {
    fn read(&mut self, count: u32) -> Option<u32> {
        let mut value = 0;
        for _ in 0..count {
            let byte = *self.data.get(self.pos / 8)?;
            value = (value << 1) | ((byte >> (7 - self.pos % 8)) & 1) as u32;
            self.pos += 1;
        }
        Some(value)
    }

    fn object_type(&mut self) -> Option<u32> {
        let t = self.read(5)?;
        if t == 31 {
            Some(32 + self.read(6)?)
        } else {
            Some(t)
        }
    }

    fn frequency_index(&mut self) -> Option<u8> {
        let index = self.read(4)?;
        if index != 15 {
            return Some(index as u8);
        }
        let frequency = self.read(24)?;
        const RATES: [u32; 13] = [96000, 88200, 64000, 48000, 44100, 32000, 24000, 22050, 16000, 12000, 11025, 8000, 7350];
        RATES.iter().enumerate().min_by_key(|(_, r)| r.abs_diff(frequency)).map(|(i, _)| i as u8)
    }
}

/// Reads an AudioSpecificConfig. For HE-AAC the core (AAC-LC) parameters are returned;
/// the decoder finds SBR/PS in the stream by itself.
pub fn parse_audio_specific_config(asc: &[u8]) -> Option<Track> {
    let mut bits = Bits { data: asc, pos: 0 };
    let mut object_type = bits.object_type()?;
    let frequency_index = bits.frequency_index()?;
    let channels = bits.read(4)? as u8;
    if object_type == 5 || object_type == 29 {
        bits.frequency_index()?;
        object_type = bits.object_type()?;
    }
    let profile = if (1..=4).contains(&object_type) { (object_type - 1) as u8 } else { 1 };
    Some(Track::Aac { profile, frequency_index, channels: if channels == 0 { 2 } else { channels } })
}

pub fn adts_header(profile: u8, frequency_index: u8, channels: u8, payload_length: usize) -> [u8; 7] {
    let length = payload_length + 7;
    [
        0xFF,
        0xF1,
        (profile << 6) | (frequency_index << 2) | ((channels >> 2) & 0x01),
        ((channels & 0x03) << 6) | ((length >> 11) & 0x03) as u8,
        ((length >> 3) & 0xFF) as u8,
        (((length & 0x07) << 5) | 0x1F) as u8,
        0xFC,
    ]
}

/// Reads one MPEG-4 descriptor: returns (tag, body).
fn descriptor(d: &[u8], pos: usize) -> Option<(u8, &[u8], usize)> {
    let tag = *d.get(pos)?;
    let mut length = 0usize;
    let mut p = pos + 1;
    for _ in 0..4 {
        let b = *d.get(p)?;
        p += 1;
        length = (length << 7) | (b & 0x7F) as usize;
        if b & 0x80 == 0 {
            break;
        }
    }
    let body = d.get(p..p + length)?;
    Some((tag, body, p + length))
}

fn parse_esds(esds: &[u8]) -> Option<Track> {
    let (tag, es, _) = descriptor(esds, 4)?;
    if tag != 0x03 || es.len() < 3 {
        return None;
    }
    let flags = es[2];
    let mut p = 3;
    if flags & 0x80 != 0 {
        p += 2;
    }
    if flags & 0x40 != 0 {
        p += 1 + *es.get(p)? as usize;
    }
    if flags & 0x20 != 0 {
        p += 2;
    }
    let (tag, config, _) = descriptor(es, p)?;
    if tag != 0x04 || config.is_empty() {
        return None;
    }
    // 0x69 / 0x6B: MPEG-2 / MPEG-1 audio (MP3).
    if matches!(config[0], 0x69 | 0x6B) {
        return Some(Track::Mp3);
    }
    let (tag, asc, _) = descriptor(config, 13)?;
    if tag != 0x05 {
        return None;
    }
    parse_audio_specific_config(asc)
}

fn parse_sample_entry(stsd: &[u8]) -> Option<Track> {
    let entry = boxes(stsd.get(8..)?).into_iter().next()?;
    match &entry.kind {
        b".mp3" => Some(Track::Mp3),
        b"mp4a" => {
            let version = u16::from_be_bytes([*entry.payload.get(8)?, *entry.payload.get(9)?]);
            let children_at = match version {
                1 => 28 + 16,
                2 => 28 + 36,
                _ => 28,
            };
            let children = entry.payload.get(children_at..)?;
            let esds = child(children, b"esds").or_else(|| path(children, &[b"wave", b"esds"]))?;
            parse_esds(esds)
        }
        _ => None,
    }
}

impl Mp4Demuxer {
    pub fn new() -> Mp4Demuxer {
        Mp4Demuxer { track: None }
    }

    /// Reads the audio track from an init section (or any data containing `moov`).
    pub fn parse_init(&mut self, data: &[u8]) -> Result<(), String> {
        let moov = child(data, b"moov").ok_or("init section without moov")?;
        for trak in boxes(moov).into_iter().filter(|b| &b.kind == b"trak") {
            if let Some(track) = path(trak.payload, &[b"mdia", b"minf", b"stbl", b"stsd"]).and_then(parse_sample_entry) {
                self.track = Some(track);
                return Ok(());
            }
        }
        Err("no supported audio track in MP4 stream (AAC or MP3 expected)".to_string())
    }

    /// Feeds one media segment and appends ADTS (or MP3) frames to `out`.
    pub fn push(&mut self, data: &[u8], out: &mut Vec<u8>) -> Result<(), String> {
        let mut moof_start = 0;
        let mut sizes: Vec<usize> = Vec::new();
        let mut data_offset: Option<i64> = None;
        for b in boxes(data) {
            match &b.kind {
                b"moov" => self.parse_init(&data[b.start..b.payload_start + b.payload.len()])?,
                b"moof" => {
                    moof_start = b.start;
                    (sizes, data_offset) = parse_moof(b.payload);
                }
                b"mdat" => {
                    let track = self.track.ok_or("MP4 segment without init section")?;
                    let mut pos = match data_offset {
                        Some(offset) => (moof_start as i64 + offset).max(0) as usize,
                        None => b.payload_start,
                    };
                    for &size in &sizes {
                        let Some(frame) = data.get(pos..pos + size) else { break };
                        if let Track::Aac { profile, frequency_index, channels } = track {
                            out.extend_from_slice(&adts_header(profile, frequency_index, channels, size));
                        }
                        out.extend_from_slice(frame);
                        pos += size;
                    }
                    sizes.clear();
                    data_offset = None;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

/// Returns the sample sizes and data offset of the first track fragment.
fn parse_moof(moof: &[u8]) -> (Vec<usize>, Option<i64>) {
    let mut sizes = Vec::new();
    let mut data_offset = None;
    let Some(traf) = child(moof, b"traf") else { return (sizes, data_offset) };
    let mut default_size = 0usize;
    for b in boxes(traf) {
        let d = b.payload;
        let flags = be_u32(d, 0).unwrap_or(0) & 0x00FF_FFFF;
        match &b.kind {
            b"tfhd" => {
                let mut p = 8;
                if flags & 0x01 != 0 {
                    p += 8;
                }
                if flags & 0x02 != 0 {
                    p += 4;
                }
                if flags & 0x08 != 0 {
                    p += 4;
                }
                if flags & 0x10 != 0 {
                    default_size = be_u32(d, p).unwrap_or(0) as usize;
                }
            }
            b"trun" => {
                let count = be_u32(d, 4).unwrap_or(0) as usize;
                let mut p = 8;
                if flags & 0x01 != 0 {
                    if data_offset.is_none() {
                        data_offset = be_u32(d, p).map(|v| v as i32 as i64);
                    }
                    p += 4;
                }
                if flags & 0x04 != 0 {
                    p += 4;
                }
                for _ in 0..count {
                    if flags & 0x100 != 0 {
                        p += 4;
                    }
                    let size = if flags & 0x200 != 0 {
                        let s = be_u32(d, p).unwrap_or(0) as usize;
                        p += 4;
                        s
                    } else {
                        default_size
                    };
                    if flags & 0x400 != 0 {
                        p += 4;
                    }
                    if flags & 0x800 != 0 {
                        p += 4;
                    }
                    if p > d.len() {
                        break;
                    }
                    sizes.push(size);
                }
            }
            _ => {}
        }
    }
    (sizes, data_offset)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sniff::adts_frame_length;

    fn mp4_box(kind: &[u8; 4], payload: &[u8]) -> Vec<u8> {
        let mut b = ((payload.len() + 8) as u32).to_be_bytes().to_vec();
        b.extend_from_slice(kind);
        b.extend_from_slice(payload);
        b
    }

    fn init_segment(asc: &[u8]) -> Vec<u8> {
        let dsi = [&[0x05, asc.len() as u8][..], asc].concat();
        let mut config = vec![0x04, (13 + dsi.len()) as u8, 0x40, 0x15];
        config.extend_from_slice(&[0; 11]);
        config.extend_from_slice(&dsi);
        let mut es = vec![0x03, (3 + config.len()) as u8, 0x00, 0x01, 0x00];
        es.extend_from_slice(&config);
        let esds = mp4_box(b"esds", &[&[0, 0, 0, 0][..], &es].concat());
        let mut mp4a_payload = vec![0u8; 28];
        mp4a_payload.extend_from_slice(&esds);
        let mp4a = mp4_box(b"mp4a", &mp4a_payload);
        let stsd = mp4_box(b"stsd", &[&[0, 0, 0, 0, 0, 0, 0, 1][..], &mp4a].concat());
        let stbl = mp4_box(b"stbl", &stsd);
        let minf = mp4_box(b"minf", &stbl);
        let mdia = mp4_box(b"mdia", &minf);
        let trak = mp4_box(b"trak", &mdia);
        let moov = mp4_box(b"moov", &trak);
        [mp4_box(b"ftyp", b"iso6"), moov].concat()
    }

    #[test]
    fn parses_audio_specific_configs() {
        // AAC-LC, 44.1 kHz, stereo.
        assert_eq!(parse_audio_specific_config(&[0x12, 0x10]), Some(Track::Aac { profile: 1, frequency_index: 4, channels: 2 }));
        // HE-AAC v1 (explicit SBR): core 24 kHz LC, extension 48 kHz.
        assert_eq!(parse_audio_specific_config(&[0x2B, 0x11, 0x88, 0x00]), Some(Track::Aac { profile: 1, frequency_index: 6, channels: 2 }));
    }

    #[test]
    fn rewraps_fragments_as_adts() {
        let mut demuxer = Mp4Demuxer::new();
        let init = init_segment(&[0x12, 0x10]);
        assert!(is_mp4(&init));
        demuxer.parse_init(&init).unwrap();

        let frames: [&[u8]; 2] = [&[1, 2, 3, 4, 5], &[6, 7, 8]];
        let tfhd = vec![0, 0, 0, 0, 0, 0, 0, 1];
        let mut trun = vec![0, 0, 0x02, 0x01, 0, 0, 0, 2, 0, 0, 0, 0];
        for f in frames {
            trun.extend_from_slice(&(f.len() as u32).to_be_bytes());
        }
        let traf = mp4_box(b"traf", &[mp4_box(b"tfhd", &tfhd), mp4_box(b"trun", &trun)].concat());
        let mut moof = mp4_box(b"moof", &traf);
        // Data offset: from the start of moof to the first sample in mdat.
        let offset = (moof.len() + 8) as u32;
        // moof header, traf header, tfhd box (8 + 8), trun header, trun version/flags + count.
        let at = 8 + 8 + 16 + 8 + 8;
        moof[at..at + 4].copy_from_slice(&offset.to_be_bytes());
        let segment = [moof, mp4_box(b"mdat", &frames.concat())].concat();

        let mut out = Vec::new();
        demuxer.push(&segment, &mut out).unwrap();
        assert_eq!(adts_frame_length(&out), Some(12));
        assert_eq!(&out[7..12], frames[0]);
        assert_eq!(adts_frame_length(&out[12..]), Some(10));
        assert_eq!(&out[19..], frames[1]);
    }
}
