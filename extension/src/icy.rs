// Copyright (C) 2026 Joncantplay
// SPDX-License-Identifier: GPL-3.0-or-later
//! Shoutcast/Icecast in-band metadata ("ICY"): strips metadata blocks from the audio
//! data and reports the current title (and a cover image, when the station sends one).

use std::io::{self, Read};

use crate::http::decode_text;

pub struct IcyReader<R: Read> {
    inner: R,
    interval: usize,
    until_meta: usize,
    last_title: Option<String>,
    on_title: Box<dyn FnMut(String, String) + Send>,
}

impl<R: Read> IcyReader<R> {
    /// `interval` is the `icy-metaint` value: audio bytes between two metadata blocks.
    /// `on_title` receives the title and a cover image URL ("" if none).
    pub fn new(inner: R, interval: usize, on_title: Box<dyn FnMut(String, String) + Send>) -> Self {
        IcyReader { inner, interval, until_meta: interval, last_title: None, on_title }
    }

    fn read_metadata(&mut self) -> io::Result<()> {
        let mut length = [0u8; 1];
        self.inner.read_exact(&mut length)?;
        let size = length[0] as usize * 16;
        if size == 0 {
            return Ok(());
        }
        let mut block = vec![0u8; size];
        self.inner.read_exact(&mut block)?;
        if let Some(title) = parse_stream_title(&block) {
            if self.last_title.as_deref() != Some(title.as_str()) {
                self.last_title = Some(title.clone());
                let cover = parse_field(&block, "StreamUrl").filter(|url| is_image_url(url)).unwrap_or_default();
                (self.on_title)(title, cover);
            }
        }
        Ok(())
    }
}

impl<R: Read> Read for IcyReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }
        if self.interval == 0 {
            return self.inner.read(buf);
        }
        if self.until_meta == 0 {
            self.read_metadata()?;
            self.until_meta = self.interval;
        }
        let want = buf.len().min(self.until_meta);
        let n = self.inner.read(&mut buf[..want])?;
        self.until_meta -= n;
        Ok(n)
    }
}

/// Extracts `StreamTitle='...';` from a metadata block.
pub fn parse_stream_title(block: &[u8]) -> Option<String> {
    parse_field(block, "StreamTitle")
}

/// Extracts `name='...';` from a metadata block.
pub fn parse_field(block: &[u8], name: &str) -> Option<String> {
    let end = block.iter().rposition(|&b| b != 0).map_or(0, |i| i + 1);
    let text = decode_text(&block[..end]);
    let key = format!("{name}='");
    let start = text.find(&key)? + key.len();
    let rest = &text[start..];
    // Values may contain apostrophes, so look for the closing `';` rather than the next quote.
    let stop = rest.find("';").or_else(|| rest.rfind('\'')).unwrap_or(rest.len());
    Some(rest[..stop].trim().to_string())
}

/// Some stations put a cover image into `StreamUrl`; others a website, which is ignored.
fn is_image_url(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    let path = lower.split(['?', '#']).next().unwrap_or("");
    (lower.starts_with("http://") || lower.starts_with("https://"))
        && [".jpg", ".jpeg", ".png", ".webp"].iter().any(|ext| path.ends_with(ext))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn parses_titles() {
        assert_eq!(parse_stream_title(b"StreamTitle='Artist - Song';StreamUrl='';\0\0"), Some("Artist - Song".into()));
        assert_eq!(parse_stream_title(b"StreamTitle='Guns N' Roses - Don't Cry';"), Some("Guns N' Roses - Don't Cry".into()));
        assert_eq!(parse_stream_title(b"StreamTitle='';"), Some(String::new()));
        assert_eq!(parse_stream_title(b"StreamUrl='x';"), None);
    }

    #[test]
    fn finds_cover_urls() {
        let block = b"StreamTitle='A - B';StreamUrl='https://cdn.example.com/covers/123.jpg?s=300';";
        assert_eq!(parse_field(block, "StreamUrl").as_deref(), Some("https://cdn.example.com/covers/123.jpg?s=300"));
        assert!(is_image_url("https://cdn.example.com/covers/123.jpg?s=300"));
        assert!(!is_image_url("https://radio.example.com/"));
        assert!(!is_image_url(""));
    }

    #[test]
    fn strips_metadata_blocks() {
        let meta = b"StreamTitle='A';";
        let mut raw = Vec::new();
        raw.extend_from_slice(b"abcd");
        raw.push(1);
        let mut block = meta.to_vec();
        block.resize(16, 0);
        raw.extend_from_slice(&block);
        raw.extend_from_slice(b"efgh");
        raw.push(0);
        raw.extend_from_slice(b"ij");

        let titles = Arc::new(Mutex::new(Vec::new()));
        let sink = titles.clone();
        let mut reader = IcyReader::new(&raw[..], 4, Box::new(move |t, _| sink.lock().unwrap().push(t)));
        let mut out = Vec::new();
        reader.read_to_end(&mut out).unwrap();
        assert_eq!(out, b"abcdefghij");
        assert_eq!(*titles.lock().unwrap(), vec!["A".to_string()]);
    }
}
