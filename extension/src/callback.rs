// Copyright (C) 2026 Joncantplay
// SPDX-License-Identifier: GPL-3.0-or-later
//! Events from the extension back to the game (`ExtensionCallback` mission event).

use std::collections::VecDeque;
use std::ffi::{c_char, c_int, CString};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Condvar, Mutex, Once, OnceLock};
use std::time::{Duration, Instant};

pub type ArmaCallback = unsafe extern "C" fn(*const c_char, *const c_char, *const c_char) -> c_int;

const EXTENSION_NAME: &str = "skywave";
const MAX_QUEUED: usize = 512;
const MAX_DATA_BYTES: usize = 8000;

static CALLBACK: Mutex<Option<ArmaCallback>> = Mutex::new(None);
static QUEUE: Mutex<VecDeque<(String, String)>> = Mutex::new(VecDeque::new());
static READY: Condvar = Condvar::new();
static START: Once = Once::new();

/// Events are only delivered while the game talks to the extension. When the game shuts
/// down it stops calling in, and calling back into a half torn down engine crashes it.
const CONTACT_TIMEOUT: Duration = Duration::from_secs(5);
static EPOCH: OnceLock<Instant> = OnceLock::new();
static LAST_CONTACT_MS: AtomicU64 = AtomicU64::new(0);

fn now_ms() -> u64 {
    EPOCH.get_or_init(Instant::now).elapsed().as_millis() as u64
}

/// Records that the game just called the extension.
pub fn touch() {
    LAST_CONTACT_MS.store(now_ms(), Ordering::Relaxed);
}

fn game_is_listening() -> bool {
    now_ms().saturating_sub(LAST_CONTACT_MS.load(Ordering::Relaxed)) < CONTACT_TIMEOUT.as_millis() as u64
}

pub fn register(callback: ArmaCallback) {
    *CALLBACK.lock().unwrap_or_else(|e| e.into_inner()) = Some(callback);
    START.call_once(|| {
        std::thread::Builder::new()
            .name("skywave-callback".into())
            .spawn(dispatch)
            .expect("failed to start callback thread");
    });
}

/// Queues an event. The oldest events are dropped if the game stops collecting them.
pub fn send(function: &str, data: String) {
    let mut queue = QUEUE.lock().unwrap_or_else(|e| e.into_inner());
    if queue.len() >= MAX_QUEUED {
        queue.pop_front();
    }
    queue.push_back((function.to_string(), data));
    READY.notify_one();
}

pub fn log(message: impl Into<String>) {
    send("log", message.into());
}

fn to_cstring(text: &str, limit: usize) -> CString {
    let mut end = text.len().min(limit);
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    CString::new(text[..end].replace('\0', "")).unwrap_or_default()
}

fn dispatch() {
    let name = to_cstring(EXTENSION_NAME, 64);
    loop {
        let (function, data) = {
            let mut queue = QUEUE.lock().unwrap_or_else(|e| e.into_inner());
            loop {
                if let Some(item) = queue.pop_front() {
                    break item;
                }
                queue = READY.wait(queue).unwrap_or_else(|e| e.into_inner());
            }
        };
        let Some(callback) = *CALLBACK.lock().unwrap_or_else(|e| e.into_inner()) else {
            continue;
        };
        if !game_is_listening() {
            continue;
        }
        let function = to_cstring(&function, 64);
        let data = to_cstring(&data, MAX_DATA_BYTES);
        // The game returns -1 while its callback queue is full; try again shortly.
        for _ in 0..100 {
            if !game_is_listening() {
                break;
            }
            let result = unsafe { callback(name.as_ptr(), function.as_ptr(), data.as_ptr()) };
            if result >= 0 {
                break;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    }
}
