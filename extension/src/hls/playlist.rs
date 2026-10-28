// Copyright (C) 2026 Joncantplay
// SPDX-License-Identifier: GPL-3.0-or-later
//! HLS playlist parsing (RFC 8216): master playlists with variants, media playlists
//! with segments, keys and init sections.

#[derive(Clone, Debug, PartialEq)]
pub struct Key {
    pub method: String,
    pub uri: String,
    pub iv: Option<[u8; 16]>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Segment {
    pub uri: String,
    pub sequence: u64,
    pub key: Option<Key>,
    /// URI of the init section (`#EXT-X-MAP`) for fragmented MP4 segments.
    pub map: Option<String>,
    /// Text after the comma of `#EXTINF`, often the current track.
    pub title: String,
}

#[derive(Debug, PartialEq)]
pub struct MediaPlaylist {
    pub target_duration: f32,
    pub segments: Vec<Segment>,
    pub ended: bool,
}

#[derive(Debug, PartialEq)]
pub struct Variant {
    pub uri: String,
    pub bandwidth: u64,
    pub codecs: String,
    pub audio_group: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct Rendition {
    pub group: String,
    pub uri: String,
    pub default: bool,
}

#[derive(Debug, PartialEq)]
pub enum Hls {
    Master { variants: Vec<Variant>, renditions: Vec<Rendition> },
    Media(MediaPlaylist),
}

/// Splits an attribute list like `METHOD=AES-128,URI="key.bin",IV=0x01`.
pub fn attributes(text: &str) -> Vec<(String, String)> {
    let mut result = Vec::new();
    let mut rest = text.trim();
    while !rest.is_empty() {
        let Some(eq) = rest.find('=') else { break };
        let key = rest[..eq].trim().trim_start_matches(',').trim().to_ascii_uppercase();
        let after = &rest[eq + 1..];
        let (value, remaining) = if let Some(quoted) = after.strip_prefix('"') {
            match quoted.find('"') {
                Some(end) => (&quoted[..end], &quoted[end + 1..]),
                None => (quoted, ""),
            }
        } else {
            match after.find(',') {
                Some(end) => (&after[..end], &after[end..]),
                None => (after, ""),
            }
        };
        result.push((key, value.trim().to_string()));
        rest = remaining.trim_start_matches(',').trim_start();
    }
    result
}

fn attribute<'a>(list: &'a [(String, String)], name: &str) -> Option<&'a str> {
    list.iter().find(|(k, _)| k == name).map(|(_, v)| v.as_str())
}

fn parse_iv(text: &str) -> Option<[u8; 16]> {
    let hex = text.strip_prefix("0x").or_else(|| text.strip_prefix("0X"))?;
    let hex = format!("{hex:0>32}");
    if hex.len() != 32 {
        return None;
    }
    let mut iv = [0u8; 16];
    for (i, byte) in iv.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).ok()?;
    }
    Some(iv)
}

pub fn parse(text: &str) -> Result<Hls, String> {
    let text = text.trim_start_matches('\u{feff}');
    if !text.trim_start().starts_with("#EXTM3U") {
        return Err("not an HLS playlist".to_string());
    }
    if text.contains("#EXT-X-STREAM-INF") {
        Ok(parse_master(text))
    } else {
        Ok(Hls::Media(parse_media(text)))
    }
}

fn parse_master(text: &str) -> Hls {
    let mut variants = Vec::new();
    let mut renditions = Vec::new();
    let mut pending: Option<Vec<(String, String)>> = None;
    for line in text.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if let Some(rest) = line.strip_prefix("#EXT-X-STREAM-INF:") {
            pending = Some(attributes(rest));
        } else if let Some(rest) = line.strip_prefix("#EXT-X-MEDIA:") {
            let attrs = attributes(rest);
            if attribute(&attrs, "TYPE") == Some("AUDIO") {
                if let (Some(group), Some(uri)) = (attribute(&attrs, "GROUP-ID"), attribute(&attrs, "URI")) {
                    renditions.push(Rendition {
                        group: group.to_string(),
                        uri: uri.to_string(),
                        default: attribute(&attrs, "DEFAULT") == Some("YES"),
                    });
                }
            }
        } else if !line.starts_with('#') {
            if let Some(attrs) = pending.take() {
                variants.push(Variant {
                    uri: line.to_string(),
                    bandwidth: attribute(&attrs, "BANDWIDTH").and_then(|b| b.parse().ok()).unwrap_or(0),
                    codecs: attribute(&attrs, "CODECS").unwrap_or("").to_ascii_lowercase(),
                    audio_group: attribute(&attrs, "AUDIO").map(str::to_string),
                });
            }
        }
    }
    Hls::Master { variants, renditions }
}

fn parse_media(text: &str) -> MediaPlaylist {
    let mut target_duration = 6.0;
    let mut sequence = 0u64;
    let mut key: Option<Key> = None;
    let mut map: Option<String> = None;
    let mut title = String::new();
    let mut ended = false;
    let mut segments = Vec::new();
    for line in text.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if let Some(v) = line.strip_prefix("#EXT-X-TARGETDURATION:") {
            target_duration = v.trim().parse().unwrap_or(target_duration);
        } else if let Some(v) = line.strip_prefix("#EXT-X-MEDIA-SEQUENCE:") {
            sequence = v.trim().parse().unwrap_or(0);
        } else if let Some(v) = line.strip_prefix("#EXTINF:") {
            title = v.split_once(',').map(|(_, t)| t.trim().to_string()).unwrap_or_default();
        } else if let Some(v) = line.strip_prefix("#EXT-X-KEY:") {
            let attrs = attributes(v);
            let method = attribute(&attrs, "METHOD").unwrap_or("NONE").to_ascii_uppercase();
            key = (method != "NONE").then(|| Key {
                method,
                uri: attribute(&attrs, "URI").unwrap_or("").to_string(),
                iv: attribute(&attrs, "IV").and_then(parse_iv),
            });
        } else if let Some(v) = line.strip_prefix("#EXT-X-MAP:") {
            map = attribute(&attributes(v), "URI").map(str::to_string);
        } else if line.starts_with("#EXT-X-ENDLIST") {
            ended = true;
        } else if !line.starts_with('#') {
            segments.push(Segment { uri: line.to_string(), sequence, key: key.clone(), map: map.clone(), title: std::mem::take(&mut title) });
            sequence += 1;
        }
    }
    MediaPlaylist { target_duration, segments, ended }
}

const VIDEO_CODECS: [&str; 6] = ["avc1", "avc3", "hvc1", "hev1", "vp09", "av01"];

/// Picks the playlist to play from a master playlist: the best audio-only variant up to
/// 320 kbit/s, otherwise an audio rendition, otherwise the cheapest variant.
pub fn choose_variant(variants: &[Variant], renditions: &[Rendition]) -> Option<String> {
    let audio_only: Vec<&Variant> = variants.iter().filter(|v| !VIDEO_CODECS.iter().any(|c| v.codecs.contains(c))).collect();
    if !audio_only.is_empty() {
        let affordable = audio_only.iter().filter(|v| v.bandwidth <= 320_000).max_by_key(|v| v.bandwidth);
        let chosen = affordable.or_else(|| audio_only.iter().min_by_key(|v| v.bandwidth));
        return chosen.map(|v| v.uri.clone());
    }
    let cheapest = variants.iter().min_by_key(|v| v.bandwidth)?;
    if let Some(group) = &cheapest.audio_group {
        let in_group: Vec<&Rendition> = renditions.iter().filter(|r| &r.group == group).collect();
        if let Some(r) = in_group.iter().find(|r| r.default).or_else(|| in_group.first()) {
            return Some(r.uri.clone());
        }
    }
    Some(cheapest.uri.clone())
}

/// Turns `#EXTINF` text into a track title when it carries one.
pub fn extinf_title(text: &str) -> Option<String> {
    let text = text.trim();
    if text.is_empty() {
        return None;
    }
    if text.contains("title=\"") {
        let attrs = attributes(text);
        let title = attribute(&attrs, "TITLE").unwrap_or("").trim();
        let artist = attribute(&attrs, "ARTIST").unwrap_or("").trim();
        return match (artist.is_empty(), title.is_empty()) {
            (false, false) => Some(format!("{artist} - {title}")),
            (true, false) => Some(title.to_string()),
            (false, true) => Some(artist.to_string()),
            (true, true) => None,
        };
    }
    (!text.contains('=')).then(|| text.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_attributes_with_quoted_commas() {
        let a = attributes("BANDWIDTH=128000,CODECS=\"mp4a.40.2,mp4a.40.5\",AUDIO=\"aac\"");
        assert_eq!(attribute(&a, "BANDWIDTH"), Some("128000"));
        assert_eq!(attribute(&a, "CODECS"), Some("mp4a.40.2,mp4a.40.5"));
        assert_eq!(attribute(&a, "AUDIO"), Some("aac"));
    }

    #[test]
    fn parses_media_playlists() {
        let text = "#EXTM3U\n#EXT-X-TARGETDURATION:6\n#EXT-X-MEDIA-SEQUENCE:100\n\
            #EXT-X-KEY:METHOD=AES-128,URI=\"key.bin\",IV=0x0000000000000000000000000000002A\n\
            #EXTINF:6.0,Artist - Song\nseg100.ts\n#EXT-X-KEY:METHOD=NONE\n#EXTINF:6.0,\nseg101.ts\n";
        let Hls::Media(m) = parse(text).unwrap() else { panic!("expected media playlist") };
        assert_eq!(m.target_duration, 6.0);
        assert!(!m.ended);
        assert_eq!(m.segments.len(), 2);
        assert_eq!(m.segments[0].sequence, 100);
        assert_eq!(m.segments[0].title, "Artist - Song");
        let key = m.segments[0].key.as_ref().unwrap();
        assert_eq!(key.uri, "key.bin");
        assert_eq!(key.iv.unwrap()[15], 0x2A);
        assert_eq!(m.segments[1].key, None);
        assert_eq!(m.segments[1].sequence, 101);
    }

    #[test]
    fn chooses_audio_variants() {
        let text = "#EXTM3U\n#EXT-X-STREAM-INF:BANDWIDTH=64000,CODECS=\"mp4a.40.5\"\nlow.m3u8\n\
            #EXT-X-STREAM-INF:BANDWIDTH=192000,CODECS=\"mp4a.40.2\"\nmid.m3u8\n\
            #EXT-X-STREAM-INF:BANDWIDTH=500000,CODECS=\"mp4a.40.2\"\nhigh.m3u8\n";
        let Hls::Master { variants, renditions } = parse(text).unwrap() else { panic!("expected master playlist") };
        assert_eq!(choose_variant(&variants, &renditions).as_deref(), Some("mid.m3u8"));
    }

    #[test]
    fn prefers_audio_rendition_over_video() {
        let text = "#EXTM3U\n#EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID=\"a\",NAME=\"en\",DEFAULT=YES,URI=\"audio.m3u8\"\n\
            #EXT-X-STREAM-INF:BANDWIDTH=900000,CODECS=\"avc1.4d401f,mp4a.40.2\",AUDIO=\"a\"\nvideo.m3u8\n";
        let Hls::Master { variants, renditions } = parse(text).unwrap() else { panic!("expected master playlist") };
        assert_eq!(choose_variant(&variants, &renditions).as_deref(), Some("audio.m3u8"));
    }

    #[test]
    fn reads_titles_from_extinf() {
        assert_eq!(extinf_title("Artist - Song").as_deref(), Some("Artist - Song"));
        assert_eq!(extinf_title("title=\"Song\",artist=\"Band\"").as_deref(), Some("Band - Song"));
        assert_eq!(extinf_title(""), None);
        assert_eq!(extinf_title("tvg-id=x"), None);
    }
}
