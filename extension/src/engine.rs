// Copyright (C) 2026 Joncantplay
// SPDX-License-Identifier: GPL-3.0-or-later
//! Radios ("sources") and the streams they listen to.

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, Once};
use std::time::Duration;

use crate::callback;
use crate::mixer;
use crate::stream::{SourceBuffer, Stream};

/// Without any position update for this long the game is gone (menu, crash): stop everything.
const ABANDONED_AFTER_SECONDS: f32 = 10.0;

struct Source {
    url: String,
}

#[derive(Default)]
struct Engine {
    streams: HashMap<String, Stream>,
    sources: HashMap<String, Source>,
}

static ENGINE: Mutex<Option<Engine>> = Mutex::new(None);
static WATCHDOG: Once = Once::new();

fn engine() -> MutexGuard<'static, Option<Engine>> {
    let mut guard = ENGINE.lock().unwrap_or_else(|e| e.into_inner());
    if guard.is_none() {
        *guard = Some(Engine::default());
    }
    guard
}

impl Engine {
    fn stop_source(&mut self, id: &str) {
        let Some(source) = self.sources.remove(id) else { return };
        mixer::remove_source(id);
        if let Some(stream) = self.streams.get(&source.url) {
            if stream.unsubscribe(id) {
                stream.stop();
                self.streams.remove(&source.url);
            }
        }
    }
}

/// Starts (or retunes) a radio.
pub fn play(id: &str, url: &str) {
    start_watchdog();
    let mut guard = engine();
    let Some(e) = guard.as_mut() else { return };
    if e.sources.get(id).is_some_and(|s| s.url == url) {
        return;
    }
    e.stop_source(id);
    let buffer = SourceBuffer::new();
    mixer::add_source(id, buffer.clone());
    let stream = e.streams.entry(url.to_string()).or_insert_with(|| Stream::start(url));
    stream.subscribe(id, buffer);
    e.sources.insert(id.to_string(), Source { url: url.to_string() });
}

pub fn stop(id: &str) {
    if let Some(e) = engine().as_mut() {
        e.stop_source(id);
    }
}

pub fn reset() {
    if let Some(e) = engine().as_mut() {
        for stream in e.streams.values() {
            stream.stop();
        }
        e.streams.clear();
        e.sources.clear();
    }
    mixer::clear();
}

fn start_watchdog() {
    WATCHDOG.call_once(|| {
        let spawned = std::thread::Builder::new().name("skywave-watchdog".into()).spawn(|| loop {
            std::thread::sleep(Duration::from_secs(1));
            let active = engine().as_ref().is_some_and(|e| !e.sources.is_empty());
            if active && mixer::seconds_since_update() > ABANDONED_AFTER_SECONDS {
                callback::log("no updates from the game, stopping all radios");
                reset();
            }
        });
        if let Err(e) = spawned {
            callback::log(format!("could not start watchdog: {e}"));
        }
    });
}
