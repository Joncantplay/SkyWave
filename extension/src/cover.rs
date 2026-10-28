// Copyright (C) 2026 Joncantplay
// SPDX-License-Identifier: GPL-3.0-or-later
//! Album covers for the track that is playing.
//!
//! The title ("Artist - Title") is looked up in MusicBrainz, an open music database; the
//! cover comes from the Cover Art Archive. MusicBrainz allows one request per second per
//! client, so lookups are spaced out.

use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::http::{Agent, Url};
use crate::logo::{fetch, fetch_image};

const SEARCH_URL: &str = "https://musicbrainz.org/ws/2/recording";
const COVER_URL: &str = "https://coverartarchive.org/release-group";
const MAX_SEARCH_BYTES: u64 = 512 * 1024;
/// Search results below this score (0..100) are probably a different song.
const MIN_SCORE: u64 = 80;
const MAX_COVER_TRIES: usize = 4;
const REQUEST_SPACING: Duration = Duration::from_millis(1100);

static LAST_SEARCH: Mutex<Option<Instant>> = Mutex::new(None);

/// Splits "Artist - Title" (also with en or em dash).
pub fn split_title(title: &str) -> Option<(String, String)> {
    for separator in [" - ", " – ", " — "] {
        if let Some((artist, song)) = title.split_once(separator) {
            let (artist, song) = (artist.trim(), song.trim());
            if !artist.is_empty() && !song.is_empty() {
                return Some((artist.to_string(), song.to_string()));
            }
        }
    }
    None
}

pub fn percent_encode(text: &str) -> String {
    let mut out = String::new();
    for byte in text.bytes() {
        if byte.is_ascii_alphanumeric() || b"-_.~".contains(&byte) {
            out.push(byte as char);
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out
}

/// Characters with a meaning in the search syntax are left out of the terms.
fn search_term(text: &str) -> String {
    text.chars().filter(|c| !"\"\\:()[]{}^~*?!+&|/".contains(*c)).collect::<String>().trim().to_string()
}

/// Release groups of well matching recordings, albums first.
pub fn release_groups(json: &serde_json::Value) -> Vec<String> {
    let mut groups: Vec<(bool, String)> = Vec::new();
    for recording in json["recordings"].as_array().into_iter().flatten() {
        if recording["score"].as_u64().unwrap_or(0) < MIN_SCORE {
            continue;
        }
        for release in recording["releases"].as_array().into_iter().flatten() {
            let group = &release["release-group"];
            if let Some(id) = group["id"].as_str() {
                if !groups.iter().any(|(_, g)| g == id) {
                    groups.push((group["primary-type"].as_str() == Some("Album"), id.to_string()));
                }
            }
        }
    }
    // Stable sort: albums first, otherwise keep the search order.
    groups.sort_by_key(|(album, _)| !*album);
    groups.into_iter().map(|(_, id)| id).collect()
}

fn wait_for_turn() {
    let mut last = LAST_SEARCH.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(previous) = *last {
        let elapsed = previous.elapsed();
        if elapsed < REQUEST_SPACING {
            std::thread::sleep(REQUEST_SPACING - elapsed);
        }
    }
    *last = Some(Instant::now());
}

pub fn find_cover(title: &str) -> Option<(Vec<u8>, &'static str)> {
    let (artist, song) = split_title(title)?;
    let query = format!("artist:\"{}\" AND recording:\"{}\"", search_term(&artist), search_term(&song));
    let url = Url::parse(&format!("{SEARCH_URL}?query={}&fmt=json&limit=5", percent_encode(&query))).ok()?;

    wait_for_turn();
    let (_, body) = fetch(&url, MAX_SEARCH_BYTES, Agent::App)?;
    let json: serde_json::Value = serde_json::from_slice(&body).ok()?;
    release_groups(&json)
        .iter()
        .take(MAX_COVER_TRIES)
        .filter_map(|id| Url::parse(&format!("{COVER_URL}/{id}/front-250")).ok())
        .find_map(|cover| fetch_image(&cover, Agent::App))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_titles() {
        assert_eq!(split_title("Daft Punk - Get Lucky"), Some(("Daft Punk".into(), "Get Lucky".into())));
        assert_eq!(split_title("AC/DC – Thunderstruck"), Some(("AC/DC".into(), "Thunderstruck".into())));
        assert_eq!(split_title("Jingle"), None);
        assert_eq!(split_title(" - Title"), None);
    }

    #[test]
    fn encodes_queries() {
        assert_eq!(percent_encode("artist:\"A B\""), "artist%3A%22A%20B%22");
        assert_eq!(percent_encode("Ö"), "%C3%96");
        assert_eq!(search_term("Song (Radio Edit)"), "Song Radio Edit");
    }

    #[test]
    fn prefers_album_release_groups() {
        let json: serde_json::Value = serde_json::from_str(
            r#"{"recordings":[
                {"score":100,"releases":[
                    {"release-group":{"id":"single-1","primary-type":"Single"}},
                    {"release-group":{"id":"album-1","primary-type":"Album"}},
                    {"release-group":{"id":"album-1","primary-type":"Album"}}]},
                {"score":40,"releases":[{"release-group":{"id":"other","primary-type":"Album"}}]}
            ]}"#,
        )
        .unwrap();
        assert_eq!(release_groups(&json), vec!["album-1".to_string(), "single-1".to_string()]);
    }
}
