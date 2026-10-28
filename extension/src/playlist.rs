// Copyright (C) 2026 Joncantplay
// SPDX-License-Identifier: GPL-3.0-or-later
//! Simple playlists (.m3u, .pls) that point to the actual stream, and HLS detection.

#[derive(Debug, PartialEq)]
pub enum Playlist {
    /// HTTP Live Streaming playlist (master or media).
    Hls,
    /// Plain list of stream addresses, in order of preference.
    Entries(Vec<String>),
}

const PLAYLIST_TYPES: &[&str] = &[
    "audio/x-mpegurl",
    "audio/mpegurl",
    "application/x-mpegurl",
    "application/vnd.apple.mpegurl",
    "audio/x-scpls",
    "audio/scpls",
];

/// Decides from content type, path and the first bytes whether a response is a playlist.
pub fn looks_like_playlist(content_type: &str, path: &str, head: &[u8]) -> bool {
    if PLAYLIST_TYPES.contains(&content_type) {
        return true;
    }
    let start = String::from_utf8_lossy(&head[..head.len().min(64)]);
    let start = start.trim_start_matches('\u{feff}').trim_start();
    if start.starts_with("#EXTM3U") || start.to_ascii_lowercase().starts_with("[playlist]") {
        return true;
    }
    if content_type.starts_with("audio/") && content_type != "audio/x-mpegurl" {
        return false;
    }
    let path = path.split('?').next().unwrap_or("").to_ascii_lowercase();
    path.ends_with(".m3u") || path.ends_with(".m3u8") || path.ends_with(".pls")
}

pub fn parse(text: &str) -> Playlist {
    let text = text.trim_start_matches('\u{feff}');
    if text.contains("#EXT-X-") {
        return Playlist::Hls;
    }
    if text.trim_start().to_ascii_lowercase().starts_with("[playlist]") {
        let mut entries: Vec<(u32, String)> = text
            .lines()
            .filter_map(|line| {
                let (key, value) = line.split_once('=')?;
                let key = key.trim().to_ascii_lowercase();
                let index = key.strip_prefix("file")?.parse::<u32>().ok()?;
                Some((index, value.trim().to_string()))
            })
            .collect();
        entries.sort_by_key(|(index, _)| *index);
        return Playlist::Entries(entries.into_iter().map(|(_, url)| url).filter(|u| !u.is_empty()).collect());
    }
    Playlist::Entries(
        text.lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(str::to_string)
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_playlists() {
        assert!(looks_like_playlist("audio/x-scpls", "/listen", b"[playlist]"));
        assert!(looks_like_playlist("text/plain", "/radio.pls", b""));
        assert!(looks_like_playlist("application/octet-stream", "/x", b"#EXTM3U\n"));
        assert!(!looks_like_playlist("audio/mpeg", "/stream.m3u", b"\xff\xfb"));
        assert!(!looks_like_playlist("audio/aac", "/live", b"\xff\xf1"));
    }

    #[test]
    fn parses_pls_and_m3u() {
        let pls = "[playlist]\nNumberOfEntries=2\nFile2=http://b/\nTitle1=x\nFile1=http://a/\n";
        assert_eq!(parse(pls), Playlist::Entries(vec!["http://a/".into(), "http://b/".into()]));
        let m3u = "#EXTM3U\n#EXTINF:-1,Radio\nhttp://a/stream\n\n";
        assert_eq!(parse(m3u), Playlist::Entries(vec!["http://a/stream".into()]));
        let hls = "#EXTM3U\n#EXT-X-STREAM-INF:BANDWIDTH=128000\nlow.m3u8\n";
        assert_eq!(parse(hls), Playlist::Hls);
    }
}
