// Copyright (C) 2026 Joncantplay
// SPDX-License-Identifier: GPL-3.0-or-later
//! MPEG transport stream demuxer: extracts the first audio track (ADTS AAC or MPEG audio)
//! and track titles from ID3 timed metadata.

const PACKET: usize = 188;

pub struct TsDemuxer {
    pmt_pid: Option<u16>,
    audio_pid: Option<u16>,
    pes: Vec<u8>,
    id3_pid: Option<u16>,
    id3_pes: Vec<u8>,
    title: Option<String>,
}

/// Returns true if `data` looks like a transport stream.
pub fn is_transport_stream(data: &[u8]) -> bool {
    data.first() == Some(&0x47) && (data.len() <= PACKET || data[PACKET] == 0x47)
}

impl TsDemuxer {
    pub fn new() -> TsDemuxer {
        TsDemuxer { pmt_pid: None, audio_pid: None, pes: Vec::new(), id3_pid: None, id3_pes: Vec::new(), title: None }
    }

    /// Feeds one segment and appends the audio elementary stream to `out`.
    pub fn push(&mut self, data: &[u8], out: &mut Vec<u8>) {
        let mut i = 0;
        while i + PACKET <= data.len() {
            if data[i] != 0x47 {
                i += 1;
                continue;
            }
            self.packet(&data[i..i + PACKET], out);
            i += PACKET;
        }
    }

    fn packet(&mut self, p: &[u8], out: &mut Vec<u8>) {
        let start = p[1] & 0x40 != 0;
        let pid = (((p[1] & 0x1F) as u16) << 8) | p[2] as u16;
        let adaptation = (p[3] >> 4) & 0x03;
        if adaptation & 0x01 == 0 {
            return;
        }
        let mut offset = 4;
        if adaptation == 3 {
            offset += 1 + p[4] as usize;
        }
        if offset >= PACKET {
            return;
        }
        let payload = &p[offset..];
        if pid == 0 {
            if start {
                self.parse_pat(payload);
            }
        } else if Some(pid) == self.pmt_pid {
            if start {
                self.parse_pmt(payload);
            }
        } else if Some(pid) == self.audio_pid {
            if start {
                self.flush(out);
            }
            self.pes.extend_from_slice(payload);
        } else if Some(pid) == self.id3_pid {
            if start {
                self.flush_id3();
            }
            self.id3_pes.extend_from_slice(payload);
        }
    }

    /// The latest track title found in the stream's ID3 metadata, once.
    pub fn take_title(&mut self) -> Option<String> {
        self.flush_id3();
        self.title.take()
    }

    fn flush_id3(&mut self) {
        if let Some(data) = pes_payload(&self.id3_pes) {
            let tag = &data[..crate::sniff::id3_length(data).min(data.len())];
            if let Some(title) = super::id3::title(tag) {
                self.title = Some(title);
            }
        }
        self.id3_pes.clear();
    }

    fn section(payload: &[u8]) -> Option<&[u8]> {
        let pointer = *payload.first()? as usize;
        let s = payload.get(1 + pointer..)?;
        if s.len() < 3 {
            return None;
        }
        let length = (((s[1] & 0x0F) as usize) << 8) | s[2] as usize;
        s.get(..3 + length)
    }

    fn parse_pat(&mut self, payload: &[u8]) {
        let Some(s) = Self::section(payload) else { return };
        let end = s.len().saturating_sub(4);
        let mut j = 8;
        while j + 4 <= end {
            let program = ((s[j] as u16) << 8) | s[j + 1] as u16;
            let pid = (((s[j + 2] & 0x1F) as u16) << 8) | s[j + 3] as u16;
            if program != 0 {
                self.pmt_pid = Some(pid);
                return;
            }
            j += 4;
        }
    }

    fn parse_pmt(&mut self, payload: &[u8]) {
        let Some(s) = Self::section(payload) else { return };
        if s.len() < 12 {
            return;
        }
        let end = s.len().saturating_sub(4);
        let info_length = (((s[10] & 0x0F) as usize) << 8) | s[11] as usize;
        let mut j = 12 + info_length;
        while j + 5 <= end {
            let stream_type = s[j];
            let pid = (((s[j + 1] & 0x1F) as u16) << 8) | s[j + 2] as u16;
            let es_info = (((s[j + 3] & 0x0F) as usize) << 8) | s[j + 4] as usize;
            // 0x0F: AAC in ADTS, 0x03/0x04: MPEG-1/2 audio.
            if self.audio_pid.is_none() && matches!(stream_type, 0x0F | 0x03 | 0x04) {
                self.audio_pid = Some(pid);
            }
            // 0x15: metadata carried in PES packets (ID3 timed metadata in HLS).
            if self.id3_pid.is_none() && stream_type == 0x15 {
                self.id3_pid = Some(pid);
            }
            j += 5 + es_info;
        }
    }

    fn flush(&mut self, out: &mut Vec<u8>) {
        if let Some(data) = pes_payload(&self.pes) {
            out.extend_from_slice(data);
        }
        self.pes.clear();
    }
}

/// The payload of a complete PES packet, without its header.
fn pes_payload(pes: &[u8]) -> Option<&[u8]> {
    if pes.len() < 9 || pes[..3] != [0, 0, 1] {
        return None;
    }
    pes.get(9 + pes[8] as usize..)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn packet(pid: u16, start: bool, payload: &[u8]) -> Vec<u8> {
        let mut p = vec![0x47, ((start as u8) << 6) | ((pid >> 8) as u8 & 0x1F), pid as u8];
        let room = PACKET - 4;
        if payload.len() < room {
            // Pad with an adaptation field of stuffing bytes.
            let stuffing = room - payload.len() - 1;
            p.push(0x30);
            p.push(stuffing as u8);
            if stuffing > 0 {
                p.push(0x00);
                p.extend(std::iter::repeat(0xFF).take(stuffing - 1));
            }
        } else {
            p.push(0x10);
        }
        p.extend_from_slice(payload);
        assert_eq!(p.len(), PACKET);
        p
    }

    fn section(table_id: u8, body: &[u8]) -> Vec<u8> {
        let length = body.len() + 5 + 4; // header after length + CRC
        let mut s = vec![0x00, table_id, 0xB0 | ((length >> 8) as u8), length as u8, 0x00, 0x01, 0xC1, 0x00, 0x00];
        s.extend_from_slice(body);
        s.extend_from_slice(&[0, 0, 0, 0]);
        s
    }

    #[test]
    fn extracts_audio_from_transport_stream() {
        let pat = section(0x00, &[0x00, 0x01, 0xF0, 0x00]); // program 1 -> PMT PID 0x1000
        let pmt = section(0x02, &[0xE1, 0x00, 0xF0, 0x00, 0x1B, 0xE1, 0x00, 0xF0, 0x00, 0x0F, 0xE1, 0x01, 0xF0, 0x00]);
        let audio: Vec<u8> = (0..300u32).map(|i| i as u8).collect();
        let mut pes = vec![0, 0, 1, 0xC0, 0, 0, 0x80, 0x80, 5, 1, 2, 3, 4, 5];
        pes.extend_from_slice(&audio);

        let mut stream = packet(0, true, &pat);
        stream.extend(packet(0x1000, true, &pmt));
        stream.extend(packet(0x0100, true, &[0xAA; 50])); // video, ignored
        stream.extend(packet(0x0101, true, &pes[..184]));
        stream.extend(packet(0x0101, false, &pes[184..]));
        stream.extend(packet(0x0101, true, &[0, 0, 1, 0xC0, 0, 0, 0x80, 0x00, 0]));

        assert!(is_transport_stream(&stream));
        let mut demuxer = TsDemuxer::new();
        let mut out = Vec::new();
        demuxer.push(&stream, &mut out);
        assert_eq!(out, audio);
    }

    #[test]
    fn reads_titles_from_id3_metadata() {
        let pmt = section(0x02, &[0xE1, 0x00, 0xF0, 0x00, 0x15, 0xE1, 0x02, 0xF0, 0x00]);
        let mut frame = b"TIT2".to_vec();
        let text = b"\x03Song";
        frame.extend_from_slice(&(text.len() as u32).to_be_bytes());
        frame.extend_from_slice(&[0, 0]);
        frame.extend_from_slice(text);
        let mut tag = b"ID3\x04\x00\x00\x00\x00\x00".to_vec();
        tag.push(frame.len() as u8);
        tag.extend_from_slice(&frame);
        let mut pes = vec![0, 0, 1, 0xBD, 0, 0, 0x80, 0x80, 5, 1, 2, 3, 4, 5];
        pes.extend_from_slice(&tag);

        let mut stream = packet(0, true, &section(0x00, &[0x00, 0x01, 0xF0, 0x00]));
        stream.extend(packet(0x1000, true, &pmt));
        stream.extend(packet(0x0102, true, &pes));
        let mut demuxer = TsDemuxer::new();
        demuxer.push(&stream, &mut Vec::new());
        assert_eq!(demuxer.take_title().as_deref(), Some("Song"));
        assert_eq!(demuxer.take_title(), None);
    }
}
