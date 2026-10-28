// Copyright (C) 2026 Joncantplay
// SPDX-License-Identifier: GPL-3.0-or-later
//! Reads the track title from ID3v2 tags that HLS packed-audio segments start with.

use crate::http::decode_text;

fn synchsafe(b: &[u8]) -> usize {
    b.iter().fold(0, |acc, &x| (acc << 7) | (x & 0x7F) as usize)
}

fn text_frame(body: &[u8]) -> Option<String> {
    let (&encoding, rest) = body.split_first()?;
    let text = match encoding {
        0 => rest.iter().map(|&b| b as char).collect(),
        1 | 2 => {
            let (little_endian, data) = match rest {
                [0xFF, 0xFE, data @ ..] => (true, data),
                [0xFE, 0xFF, data @ ..] => (false, data),
                data => (encoding == 1, data),
            };
            let units: Vec<u16> = data
                .chunks_exact(2)
                .map(|c| if little_endian { u16::from_le_bytes([c[0], c[1]]) } else { u16::from_be_bytes([c[0], c[1]]) })
                .collect();
            String::from_utf16_lossy(&units)
        }
        _ => decode_text(rest),
    };
    let text = text.trim_matches(char::from(0)).trim().to_string();
    (!text.is_empty()).then_some(text)
}

/// Returns "Artist - Title" (or what is available) from an ID3v2.3/2.4 tag.
pub fn title(tag: &[u8]) -> Option<String> {
    if tag.len() < 10 || &tag[..3] != b"ID3" {
        return None;
    }
    let version = tag[3];
    let flags = tag[5];
    let end = (10 + synchsafe(&tag[6..10])).min(tag.len());
    let mut p = 10;
    if flags & 0x40 != 0 && p + 4 <= end {
        p += if version >= 4 { synchsafe(&tag[p..p + 4]) } else { 4 + synchsafe_free(&tag[p..p + 4]) };
    }
    let mut song = None;
    let mut artist = None;
    while p + 10 <= end {
        let id = &tag[p..p + 4];
        if id[0] == 0 {
            break;
        }
        let size = if version >= 4 { synchsafe(&tag[p + 4..p + 8]) } else { synchsafe_free(&tag[p + 4..p + 8]) };
        let Some(body) = tag.get(p + 10..p + 10 + size) else { break };
        match id {
            b"TIT2" => song = text_frame(body),
            b"TPE1" => artist = text_frame(body),
            _ => {}
        }
        p += 10 + size;
    }
    match (artist, song) {
        (Some(a), Some(s)) => Some(format!("{a} - {s}")),
        (None, Some(s)) => Some(s),
        (Some(a), None) => Some(a),
        (None, None) => None,
    }
}

fn synchsafe_free(b: &[u8]) -> usize {
    u32::from_be_bytes([b[0], b[1], b[2], b[3]]) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(id: &[u8; 4], text: &str) -> Vec<u8> {
        let body = [&[3u8][..], text.as_bytes()].concat();
        let mut f = id.to_vec();
        f.extend_from_slice(&(body.len() as u32).to_be_bytes());
        f.extend_from_slice(&[0, 0]);
        f.extend_from_slice(&body);
        f
    }

    #[test]
    fn reads_artist_and_title() {
        let frames = [frame(b"TPE1", "Röyksopp"), frame(b"TIT2", "Eple"), frame(b"PRIV", "x")].concat();
        let mut tag = b"ID3\x03\x00\x00".to_vec();
        let size = frames.len();
        tag.extend_from_slice(&[(size >> 21) as u8 & 0x7F, (size >> 14) as u8 & 0x7F, (size >> 7) as u8 & 0x7F, size as u8 & 0x7F]);
        tag.extend_from_slice(&frames);
        assert_eq!(title(&tag).as_deref(), Some("Röyksopp - Eple"));
    }

    #[test]
    fn reads_utf16_frames() {
        assert_eq!(text_frame(&[1, 0xFF, 0xFE, b'H', 0, b'i', 0]).as_deref(), Some("Hi"));
        assert_eq!(text_frame(&[0, 0]), None);
    }
}
