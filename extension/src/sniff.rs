// Copyright (C) 2026 Joncantplay
// SPDX-License-Identifier: GPL-3.0-or-later
//! Detects the audio codec of a stream from its first bytes or its content type.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Codec {
    Mp3,
    /// AAC in ADTS framing (AAC-LC, HE-AAC v1/v2).
    Aac,
}

impl Codec {
    pub fn name(self) -> &'static str {
        match self {
            Codec::Mp3 => "MP3",
            Codec::Aac => "AAC",
        }
    }
}

pub fn from_content_type(content_type: &str) -> Option<Codec> {
    match content_type {
        "audio/mpeg" | "audio/mp3" | "audio/mpeg3" | "audio/x-mpeg" | "audio/x-mp3" => Some(Codec::Mp3),
        "audio/aac" | "audio/aacp" | "audio/x-aac" | "audio/x-aacp" | "audio/adts" => Some(Codec::Aac),
        _ => None,
    }
}

/// Picks the codec: the data wins over the content type because servers mislabel streams.
pub fn detect(content_type: &str, head: &[u8]) -> Option<Codec> {
    sniff(head).or_else(|| from_content_type(content_type))
}

/// Length of an ID3v2 tag at the start of `data`, or 0.
pub fn id3_length(data: &[u8]) -> usize {
    if data.len() >= 10 && &data[..3] == b"ID3" {
        let size = data[6..10].iter().fold(0usize, |acc, &b| (acc << 7) | (b & 0x7f) as usize);
        let footer = if data[5] & 0x10 != 0 { 10 } else { 0 };
        10 + size + footer
    } else {
        0
    }
}

/// Frame length of an ADTS header at `data[0]`, if it is one.
pub fn adts_frame_length(data: &[u8]) -> Option<usize> {
    if data.len() < 7 || data[0] != 0xFF || data[1] & 0xF6 != 0xF0 {
        return None;
    }
    if (data[2] >> 2) & 0x0F > 12 {
        return None;
    }
    let length = ((data[3] as usize & 0x03) << 11) | ((data[4] as usize) << 3) | (data[5] as usize >> 5);
    (length >= 7).then_some(length)
}

/// Frame length of an MPEG audio Layer III header at `data[0]`, if it is one.
pub fn mp3_frame_length(data: &[u8]) -> Option<usize> {
    if data.len() < 4 || data[0] != 0xFF || data[1] & 0xE0 != 0xE0 {
        return None;
    }
    let version = (data[1] >> 3) & 0x03; // 3 = MPEG-1, 2 = MPEG-2, 0 = MPEG-2.5
    let layer = (data[1] >> 1) & 0x03; // 1 = Layer III
    if version == 1 || layer != 1 {
        return None;
    }
    let bitrate_index = (data[2] >> 4) as usize;
    let rate_index = ((data[2] >> 2) & 0x03) as usize;
    if bitrate_index == 0 || bitrate_index == 15 || rate_index == 3 {
        return None;
    }
    const BITRATES_V1: [u32; 15] = [0, 32, 40, 48, 56, 64, 80, 96, 112, 128, 160, 192, 224, 256, 320];
    const BITRATES_V2: [u32; 15] = [0, 8, 16, 24, 32, 40, 48, 56, 64, 80, 96, 112, 128, 144, 160];
    const RATES: [[u32; 3]; 3] = [[44100, 48000, 32000], [22050, 24000, 16000], [11025, 12000, 8000]];
    let padding = ((data[2] >> 1) & 0x01) as u32;
    let (bitrate, rate, factor) = match version {
        3 => (BITRATES_V1[bitrate_index], RATES[0][rate_index], 144_000),
        2 => (BITRATES_V2[bitrate_index], RATES[1][rate_index], 72_000),
        _ => (BITRATES_V2[bitrate_index], RATES[2][rate_index], 72_000),
    };
    Some((factor * bitrate / rate + padding) as usize)
}

/// Finds the first position where `frame_length` matches twice in a row
/// (or once when the data ends right after the frame).
pub fn find_sync(data: &[u8], frame_length: fn(&[u8]) -> Option<usize>) -> Option<usize> {
    (0..data.len()).find(|&i| match frame_length(&data[i..]) {
        Some(length) => {
            let next = i + length;
            if next + 4 <= data.len() {
                frame_length(&data[next..]).is_some()
            } else {
                next >= data.len()
            }
        }
        None => false,
    })
}

pub fn sniff(head: &[u8]) -> Option<Codec> {
    let data = &head[id3_length(head).min(head.len())..];
    let aac = find_sync(data, adts_frame_length);
    let mp3 = find_sync(data, mp3_frame_length);
    match (aac, mp3) {
        (Some(a), Some(m)) => Some(if a <= m { Codec::Aac } else { Codec::Mp3 }),
        (Some(_), None) => Some(Codec::Aac),
        (None, Some(_)) => Some(Codec::Mp3),
        (None, None) => None,
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// Builds an ADTS header (AAC-LC, 44.1 kHz, stereo) for a frame of `length` bytes.
    pub fn adts_header(length: usize) -> [u8; 7] {
        [
            0xFF,
            0xF1,
            0x50,
            0x80 | ((length >> 11) & 0x03) as u8,
            ((length >> 3) & 0xFF) as u8,
            (((length & 0x07) << 5) | 0x1F) as u8,
            0xFC,
        ]
    }

    #[test]
    fn detects_adts() {
        let mut data = vec![0x00, 0x12];
        for _ in 0..3 {
            data.extend_from_slice(&adts_header(20));
            data.extend_from_slice(&[0u8; 13]);
        }
        assert_eq!(adts_frame_length(&data[2..]), Some(20));
        assert_eq!(sniff(&data), Some(Codec::Aac));
        assert_eq!(find_sync(&data, adts_frame_length), Some(2));
    }

    #[test]
    fn detects_mp3() {
        // MPEG-1 Layer III, 128 kbit/s, 44.1 kHz: 417 bytes per frame.
        let header = [0xFF, 0xFB, 0x90, 0x64];
        assert_eq!(mp3_frame_length(&header), Some(417));
        let mut data = b"ID3\x03\x00\x00\x00\x00\x00\x02xx".to_vec();
        for _ in 0..3 {
            data.extend_from_slice(&header);
            data.extend_from_slice(&[0u8; 413]);
        }
        assert_eq!(sniff(&data), Some(Codec::Mp3));
    }

    #[test]
    fn falls_back_to_content_type() {
        assert_eq!(detect("audio/aacp", b"garbage"), Some(Codec::Aac));
        assert_eq!(detect("audio/ogg", b"OggS"), None);
    }
}
