// Copyright (C) 2026 Joncantplay
// SPDX-License-Identifier: GPL-3.0-or-later
//! Finds and downloads station logos.
//!
//! Without an explicit logo URL the station's website is checked: the `icy-url` the
//! stream announces, otherwise the main domain of the stream host. The page's icons
//! (apple-touch-icon, large favicons, og:image) are tried from best to worst.
//! Logos are handed to the game as `data:` URLs in chunks, because the game's
//! browser control only shows embedded images.

use std::collections::HashMap;
use std::io::Read;
use std::sync::Mutex;

use crate::args::sqf_string;
use crate::callback;
use crate::http::{self, decode_text, Abort, Agent, Url};

/// Characters per chunk returned by `logoChunk`; stays well below the game's output buffer.
pub const CHUNK_CHARS: usize = 8000;
const MAX_PAGE_BYTES: u64 = 1024 * 1024;
const MAX_IMAGE_BYTES: u64 = 2 * 1024 * 1024;
const MIN_IMAGE_BYTES: usize = 64;

enum Logo {
    Loading,
    Ready(String),
    Missing,
}

static LOGOS: Mutex<Option<HashMap<String, Logo>>> = Mutex::new(None);

fn with_logos<T>(f: impl FnOnce(&mut HashMap<String, Logo>) -> T) -> T {
    let mut guard = LOGOS.lock().unwrap_or_else(|e| e.into_inner());
    f(guard.get_or_insert_with(HashMap::new))
}

/// Starts looking up the logo for `key` unless that already happened.
/// Returns "loading", "ready" or "missing".
pub fn request(key: &str, addresses: &str, logo_url: &str) -> String {
    let (addresses, logo_url) = (addresses.to_string(), logo_url.to_string());
    start(key, move || find_logo(&addresses, &logo_url))
}

/// Starts looking up an album cover for a track title ("Artist - Title").
/// Same states and delivery as logos.
pub fn request_cover(key: &str, title: &str) -> String {
    let title = title.to_string();
    start(key, move || crate::cover::find_cover(&title))
}

/// Runs `job` once per key on its own thread and keeps the image as a data URL.
fn start(key: &str, job: impl FnOnce() -> Option<(Vec<u8>, &'static str)> + Send + 'static) -> String {
    let started = with_logos(|logos| match logos.get(key) {
        Some(Logo::Loading) => Some("loading"),
        Some(Logo::Ready(_)) => Some("ready"),
        Some(Logo::Missing) => Some("missing"),
        None => {
            logos.insert(key.to_string(), Logo::Loading);
            None
        }
    });
    if let Some(state) = started {
        return state.to_string();
    }

    let failed_key = key.to_string();
    let key = key.to_string();
    let spawned = std::thread::Builder::new().name("skywave-image".into()).spawn(move || {
        let result = job().map(|(bytes, mime)| format!("data:{mime};base64,{}", base64(&bytes)));
        let chunks = result.as_ref().map_or(0, |data| data.len().div_ceil(CHUNK_CHARS));
        with_logos(|logos| {
            logos.insert(key.clone(), result.map_or(Logo::Missing, Logo::Ready));
        });
        let state = if chunks > 0 { "ready" } else { "missing" };
        callback::send("logo", format!("[{},{},{chunks}]", sqf_string(&key), sqf_string(state)));
    });
    if spawned.is_err() {
        with_logos(|logos| logos.insert(failed_key, Logo::Missing));
        return "missing".to_string();
    }
    "loading".to_string()
}

/// Returns one chunk of a finished logo's data URL.
pub fn chunk(key: &str, index: usize) -> Option<String> {
    with_logos(|logos| match logos.get(key) {
        Some(Logo::Ready(data)) => {
            let start = index * CHUNK_CHARS;
            (start < data.len()).then(|| data[start..(start + CHUNK_CHARS).min(data.len())].to_string())
        }
        _ => None,
    })
}

fn find_logo(addresses: &str, logo_url: &str) -> Option<(Vec<u8>, &'static str)> {
    if !logo_url.trim().is_empty() {
        // The override is either the image itself or a page to search for one.
        let url = Url::parse(logo_url).ok()?;
        return fetch_image(&url, Agent::Player).or_else(|| logo_from_page(&url));
    }
    let stream = Url::parse(addresses.split_whitespace().next()?).ok()?;
    let mut pages = Vec::new();
    if let Some(home) = announced_homepage(&stream) {
        pages.push(home);
    }
    let main = Url { https: true, host: main_domain(&stream.host), port: 443, path: "/".to_string() };
    if !pages.contains(&main) {
        pages.push(main);
    }
    pages.iter().find_map(logo_from_page)
}

/// The station website a stream announces in its `icy-url` header.
fn announced_homepage(stream: &Url) -> Option<Url> {
    let response = http::get(stream, &Abort::new()).ok()?;
    let site = response.header("icy-url")?.trim().to_string();
    if site.is_empty() {
        return None;
    }
    let site = if site.contains("://") { site } else { format!("https://{site}") };
    Url::parse(&site).ok()
}

/// Reduces a host to its main domain: `ice1.somafm.com` becomes `somafm.com`.
pub fn main_domain(host: &str) -> String {
    if host.parse::<std::net::IpAddr>().is_ok() {
        return host.to_string();
    }
    let labels: Vec<&str> = host.split('.').filter(|l| !l.is_empty()).collect();
    if labels.len() <= 2 {
        return labels.join(".");
    }
    // Second-level registries like co.uk or com.au need three labels.
    let second = labels[labels.len() - 2];
    let keep = if matches!(second, "co" | "com" | "org" | "net" | "ac" | "gov" | "edu") && labels[labels.len() - 1].len() == 2 { 3 } else { 2 };
    labels[labels.len() - keep.min(labels.len())..].join(".")
}

pub fn fetch(url: &Url, limit: u64, agent: Agent) -> Option<(Url, Vec<u8>)> {
    let response = http::get_as(url, &Abort::new(), agent).ok()?;
    if response.status != 200 {
        return None;
    }
    let final_url = response.url.clone();
    let mut data = Vec::new();
    response.body.take(limit).read_to_end(&mut data).ok()?;
    Some((final_url, data))
}

pub fn fetch_image(url: &Url, agent: Agent) -> Option<(Vec<u8>, &'static str)> {
    let (_, data) = fetch(url, MAX_IMAGE_BYTES, agent)?;
    if data.len() < MIN_IMAGE_BYTES {
        return None;
    }
    let mime = image_type(&data)?;
    Some((data, mime))
}

fn logo_from_page(page: &Url) -> Option<(Vec<u8>, &'static str)> {
    let (final_url, html) = fetch(page, MAX_PAGE_BYTES, Agent::Player)?;
    let mut candidates = icon_candidates(&decode_text(&html));
    candidates.push((20, "/apple-touch-icon.png".to_string()));
    candidates.push((10, "/favicon.ico".to_string()));
    candidates.sort_by(|a, b| b.0.cmp(&a.0));
    candidates.iter().filter_map(|(_, href)| final_url.join(href).ok()).find_map(|url| fetch_image(&url, Agent::Player))
}

/// Recognises common image formats by their first bytes.
pub fn image_type(data: &[u8]) -> Option<&'static str> {
    if data.starts_with(b"\x89PNG") {
        Some("image/png")
    } else if data.starts_with(&[0xFF, 0xD8, 0xFF]) {
        Some("image/jpeg")
    } else if data.starts_with(b"GIF8") {
        Some("image/gif")
    } else if data.len() > 12 && &data[..4] == b"RIFF" && &data[8..12] == b"WEBP" {
        Some("image/webp")
    } else if data.starts_with(&[0, 0, 1, 0]) {
        Some("image/x-icon")
    } else {
        let start = String::from_utf8_lossy(&data[..data.len().min(512)]).to_ascii_lowercase();
        start.contains("<svg").then_some("image/svg+xml")
    }
}

/// Parses `name="value"` pairs of an HTML tag (text between `<` and `>`).
fn tag_attributes(tag: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let chars: Vec<char> = tag.chars().collect();
    let mut i = 0;
    // Skip the tag name.
    while i < chars.len() && !chars[i].is_whitespace() {
        i += 1;
    }
    while i < chars.len() {
        while i < chars.len() && (chars[i].is_whitespace() || chars[i] == '/') {
            i += 1;
        }
        let start = i;
        while i < chars.len() && !chars[i].is_whitespace() && chars[i] != '=' && chars[i] != '/' {
            i += 1;
        }
        let name: String = chars[start..i].iter().collect::<String>().to_ascii_lowercase();
        if name.is_empty() {
            i += 1;
            continue;
        }
        while i < chars.len() && chars[i].is_whitespace() {
            i += 1;
        }
        let mut value = String::new();
        if i < chars.len() && chars[i] == '=' {
            i += 1;
            while i < chars.len() && chars[i].is_whitespace() {
                i += 1;
            }
            if i < chars.len() && (chars[i] == '"' || chars[i] == '\'') {
                let quote = chars[i];
                i += 1;
                let start = i;
                while i < chars.len() && chars[i] != quote {
                    i += 1;
                }
                value = chars[start..i].iter().collect();
                i += 1;
            } else {
                let start = i;
                while i < chars.len() && !chars[i].is_whitespace() {
                    i += 1;
                }
                value = chars[start..i].iter().collect();
            }
        }
        result.insert(name, value.replace("&amp;", "&"));
    }
    result
}

/// Collects logo candidates from a web page with a score; higher is better.
pub fn icon_candidates(html: &str) -> Vec<(u32, String)> {
    const OG_IMAGE_SCORE: u32 = 150;
    let lower = html.to_ascii_lowercase();
    let mut result = Vec::new();
    let mut og_width: Option<u32> = None;
    let mut og_height: Option<u32> = None;
    let mut pos = 0;
    while let Some(offset) = lower[pos..].find('<') {
        let start = pos + offset + 1;
        let Some(length) = lower[start..].find('>') else { break };
        let end = start + length;
        pos = end;
        let tag_lower = &lower[start..end];
        if !(tag_lower.starts_with("link") || tag_lower.starts_with("meta")) {
            continue;
        }
        let attrs = tag_attributes(&html[start..end]);
        let get = |name: &str| attrs.get(name).map(|v| v.trim().to_string()).unwrap_or_default();
        if tag_lower.starts_with("link") {
            let rel = get("rel").to_ascii_lowercase();
            let href = get("href");
            if href.is_empty() || !rel.contains("icon") {
                continue;
            }
            let size = get("sizes")
                .split_whitespace()
                .filter_map(|s| s.split(['x', 'X']).next()?.parse::<u32>().ok())
                .max()
                .unwrap_or(0)
                .min(512);
            let score = if rel.contains("apple-touch-icon") {
                300 + size
            } else if href.to_ascii_lowercase().ends_with(".svg") {
                250
            } else {
                100 + size
            };
            result.push((score, href));
        } else {
            let keys = [get("property").to_ascii_lowercase(), get("name").to_ascii_lowercase()];
            let content = get("content");
            if content.is_empty() {
                continue;
            }
            if keys.iter().any(|k| k == "og:image" || k == "og:image:url" || k == "og:image:secure_url") {
                result.push((OG_IMAGE_SCORE, content));
            } else if keys.iter().any(|k| k == "og:image:width") {
                og_width = content.parse().ok();
            } else if keys.iter().any(|k| k == "og:image:height") {
                og_height = content.parse().ok();
            } else if keys.iter().any(|k| k == "msapplication-tileimage") {
                result.push((120, content));
            }
        }
    }
    // A large square share image is usually the station's own logo, while the app icon can be
    // the hosting platform's (e.g. laut.fm). Wide share images stay behind the icons.
    let declared_square = matches!((og_width, og_height), (Some(w), Some(h)) if w == h && w >= 200);
    for candidate in result.iter_mut().filter(|(score, _)| *score == OG_IMAGE_SCORE) {
        if declared_square || size_in_url(&candidate.1).is_some_and(|(w, h)| w == h && w >= 200) {
            candidate.0 = 900;
        }
    }
    result
}

/// Finds a size like `640x640` in an image address (some sites only state it there).
fn size_in_url(url: &str) -> Option<(u32, u32)> {
    let bytes = url.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b != b'x' || i == 0 || !bytes[i - 1].is_ascii_digit() {
            continue;
        }
        let start = bytes[..i].iter().rposition(|c| !c.is_ascii_digit()).map_or(0, |p| p + 1);
        let end = bytes[i + 1..].iter().position(|c| !c.is_ascii_digit()).map_or(bytes.len(), |p| i + 1 + p);
        if end > i + 1 {
            if let (Ok(w), Ok(h)) = (url[start..i].parse(), url[i + 1..end].parse()) {
                return Some((w, h));
            }
        }
    }
    None
}

pub fn base64(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b = [chunk[0], *chunk.get(1).unwrap_or(&0), *chunk.get(2).unwrap_or(&0)];
        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
        out.push(TABLE[(n >> 18) as usize & 63] as char);
        out.push(TABLE[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 { TABLE[(n >> 6) as usize & 63] as char } else { '=' });
        out.push(if chunk.len() > 2 { TABLE[n as usize & 63] as char } else { '=' });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_base64() {
        assert_eq!(base64(b""), "");
        assert_eq!(base64(b"f"), "Zg==");
        assert_eq!(base64(b"fo"), "Zm8=");
        assert_eq!(base64(b"foobar"), "Zm9vYmFy");
    }

    #[test]
    fn finds_main_domains() {
        assert_eq!(main_domain("ice1.somafm.com"), "somafm.com");
        assert_eq!(main_domain("stream.radioparadise.com"), "radioparadise.com");
        assert_eq!(main_domain("a.files.bbci.co.uk"), "bbci.co.uk");
        assert_eq!(main_domain("example.org"), "example.org");
        assert_eq!(main_domain("10.0.0.1"), "10.0.0.1");
    }

    #[test]
    fn ranks_icon_candidates() {
        let html = r#"<html><head>
            <link rel="icon" href="/favicon-32.png" sizes="32x32">
            <LINK REL="apple-touch-icon" HREF='/touch.png' sizes="180x180" />
            <meta property="og:image" content="https://cdn.example.com/share.jpg">
            <meta property="og:image:width" content="1200">
            <link rel="stylesheet" href="/style.css">
            <link rel="icon" type="image/svg+xml" href="/logo.svg">
        </head></html>"#;
        let mut c = icon_candidates(html);
        c.sort_by(|a, b| b.0.cmp(&a.0));
        let order: Vec<&str> = c.iter().map(|(_, h)| h.as_str()).collect();
        assert_eq!(order, vec!["/touch.png", "/logo.svg", "https://cdn.example.com/share.jpg", "/favicon-32.png"]);
    }

    #[test]
    fn prefers_square_share_images() {
        let html = r#"<link rel="apple-touch-icon" href="/platform-icon.png">
            <meta property="og:image" content="https://cdn.example.com/station.png">
            <meta property="og:image:width" content="640">
            <meta property="og:image:height" content="640">"#;
        let mut c = icon_candidates(html);
        c.sort_by(|a, b| b.0.cmp(&a.0));
        assert_eq!(c[0].1, "https://cdn.example.com/station.png");

        let html = r#"<link rel="apple-touch-icon" href="/platform-icon.png">
            <meta property="og:image" content="https://assets.example.com/abc?t=_640x640">"#;
        let mut c = icon_candidates(html);
        c.sort_by(|a, b| b.0.cmp(&a.0));
        assert_eq!(c[0].1, "https://assets.example.com/abc?t=_640x640");
        assert_eq!(size_in_url("https://x.org/img_1200x630.jpg"), Some((1200, 630)));
        assert_eq!(size_in_url("https://x.org/box.jpg"), None);
    }

    #[test]
    fn recognises_images() {
        assert_eq!(image_type(b"\x89PNG\r\n\x1a\n"), Some("image/png"));
        assert_eq!(image_type(&[0xFF, 0xD8, 0xFF, 0xE0]), Some("image/jpeg"));
        assert_eq!(image_type(b"<?xml version=\"1.0\"?><svg xmlns="), Some("image/svg+xml"));
        assert_eq!(image_type(b"<html>not an image</html>"), None);
    }

    #[test]
    fn hands_out_chunks() {
        let data = "x".repeat(CHUNK_CHARS + 10);
        with_logos(|logos| logos.insert("test".into(), Logo::Ready(data.clone())));
        assert_eq!(chunk("test", 0).map(|c| c.len()), Some(CHUNK_CHARS));
        assert_eq!(chunk("test", 1).map(|c| c.len()), Some(10));
        assert_eq!(chunk("test", 2), None);
        assert_eq!(request("test", "", ""), "ready");
    }
}
