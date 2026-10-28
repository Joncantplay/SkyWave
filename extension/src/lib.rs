// Copyright (C) 2026 Joncantplay
// SPDX-License-Identifier: GPL-3.0-or-later
//! SkyWave audio extension for Arma 3.
//!
//! The game calls in through the `RVExtension*` exports (`callExtension` in SQF).
//! Functions:
//! - `version`: returns the extension version.
//! - `play [id, url]`: starts or retunes the radio `id`. The URL may point to an
//!   Icecast/Shoutcast stream, an .m3u/.pls playlist or an HLS playlist (.m3u8).
//!   Several URLs separated by spaces are tried in order (fallbacks).
//! - `stop [id]`: stops the radio `id`.
//! - `reset`: stops all radios.
//! - `logo [key, addresses, logo url]`: starts finding a station logo; answers
//!   "loading", "ready" or "missing". A `logo` event follows when it is done.
//! - `clipboard [text]`: copies text to the Windows clipboard.
//! - `cover [key, title]`: like `logo`, but finds the album cover for "Artist - Title"
//!   (MusicBrainz / Cover Art Archive).
//! - `logoChunk [key, index]`: part `index` of a logo or cover as a `data:` URL.
//! - `frame [listener, master, sources]`: positions for this frame, see [`parse_frame`].
//!
//! Status, track titles and log lines come back through `ExtensionCallback`.

mod args;
mod callback;
mod clipboard;
mod cover;
mod decode;
mod engine;
mod faad;
mod hls;
mod http;
mod icy;
mod logo;
mod mixer;
mod output;
mod playlist;
mod resample;
mod sniff;
mod stream;

use std::ffi::{c_char, c_int, c_uint};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Once;

use mixer::{Listener, SourceParams};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

static STARTUP: Once = Once::new();
static PINNED: Once = Once::new();

#[link(name = "kernel32")]
extern "system" {
    fn GetModuleHandleExW(flags: u32, module_name: *const u16, module: *mut *mut std::ffi::c_void) -> i32;
}

/// Keeps this DLL loaded until the process ends.
///
/// The game unloads extensions while shutting down, but the audio, stream and callback
/// threads may still be running; executing unloaded code would crash the game on exit.
fn pin_module() {
    PINNED.call_once(|| {
        const PIN: u32 = 0x1;
        const FROM_ADDRESS: u32 = 0x4;
        let mut module = std::ptr::null_mut();
        let address = pin_module as *const () as *const u16;
        unsafe { GetModuleHandleExW(PIN | FROM_ADDRESS, address, &mut module) };
    });
}

/// Runs at the start of every call from the game.
fn on_call() {
    pin_module();
    callback::touch();
}

fn startup() {
    STARTUP.call_once(|| {
        std::panic::set_hook(Box::new(|info| callback::log(format!("internal error: {info}"))));
        callback::log(format!("SkyWave {VERSION} - AAC decoding: code from FAAD2 is copyright (c) Nero AG, www.nero.com"));
        output::start();
    });
}

fn parse_floats(text: &str) -> Result<Vec<f32>, String> {
    text.split(',').map(|v| v.trim().parse::<f32>().map_err(|_| format!("invalid number '{v}'"))).collect()
}

fn parse_vec3(parts: &[&str]) -> Result<[f32; 3], String> {
    let mut v = [0.0; 3];
    for (slot, text) in v.iter_mut().zip(parts) {
        *slot = text.trim().parse().map_err(|_| format!("invalid number '{text}'"))?;
    }
    Ok(v)
}

/// Parses the `frame` arguments:
/// - listener: `"px,py,pz,fx,fy,fz,ux,uy,uz"` (position ASL, forward and up vectors)
/// - master volume: `"0.8"`
/// - sources: `"id|x|y|z|volume|muffle|inside|range;..."`
pub fn parse_frame(args: &[String]) -> Result<(Listener, f32, Vec<(String, SourceParams)>), String> {
    let [listener, master, sources] = args else {
        return Err("frame expects 3 arguments".to_string());
    };
    let l = parse_floats(listener)?;
    if l.len() != 9 {
        return Err("listener needs 9 numbers".to_string());
    }
    let listener = Listener { position: [l[0], l[1], l[2]], forward: [l[3], l[4], l[5]], up: [l[6], l[7], l[8]] };
    let master: f32 = master.trim().parse().map_err(|_| format!("invalid master volume '{master}'"))?;
    let mut list = Vec::new();
    for entry in sources.split(';').filter(|s| !s.trim().is_empty()) {
        let fields: Vec<&str> = entry.split('|').collect();
        if fields.len() != 8 {
            return Err(format!("invalid source '{entry}'"));
        }
        let number = |i: usize| fields[i].trim().parse::<f32>().map_err(|_| format!("invalid number '{}'", fields[i]));
        list.push((
            fields[0].to_string(),
            SourceParams {
                position: parse_vec3(&fields[1..4])?,
                volume: number(4)?,
                muffle: number(5)?,
                inside: fields[6].trim() == "1",
                range: number(7)?,
            },
        ));
    }
    Ok((listener, master, list))
}

fn dispatch(function: &str, args: &[String]) -> Result<String, String> {
    match function {
        "version" => Ok(VERSION.to_string()),
        "reset" => {
            engine::reset();
            Ok(String::new())
        }
        "play" => {
            let [id, url] = args else {
                return Err("play expects [id, url]".to_string());
            };
            startup();
            engine::play(id, url);
            Ok(String::new())
        }
        "stop" => {
            let [id] = args else {
                return Err("stop expects [id]".to_string());
            };
            engine::stop(id);
            Ok(String::new())
        }
        "frame" => {
            let (listener, master, sources) = parse_frame(args)?;
            mixer::update(listener, master, sources);
            Ok(String::new())
        }
        "logo" => {
            let [key, addresses, logo_url] = args else {
                return Err("logo expects [key, addresses, logo url]".to_string());
            };
            Ok(logo::request(key, addresses, logo_url))
        }
        "clipboard" => {
            let [text] = args else {
                return Err("clipboard expects [text]".to_string());
            };
            clipboard::copy(text).map(|()| String::new())
        }
        "cover" => {
            let [key, title] = args else {
                return Err("cover expects [key, title]".to_string());
            };
            Ok(logo::request_cover(key, title))
        }
        "logoChunk" => {
            let [key, index] = args else {
                return Err("logoChunk expects [key, index]".to_string());
            };
            let index: usize = index.trim().parse().map_err(|_| format!("invalid chunk index '{index}'"))?;
            logo::chunk(key, index).ok_or_else(|| "no such logo chunk".to_string())
        }
        _ => Err(format!("unknown function '{function}'")),
    }
}

/// Runs a command without letting a panic cross into the game.
fn guarded(function: &str, args: &[String]) -> (String, c_int) {
    match catch_unwind(AssertUnwindSafe(|| dispatch(function, args))) {
        Ok(Ok(result)) => (result, 0),
        Ok(Err(error)) => (error, 1),
        Err(_) => ("internal error".to_string(), 2),
    }
}

/// # Safety
/// Called by Arma with a writable buffer of `output_size` bytes.
#[no_mangle]
pub unsafe extern "C" fn RVExtensionVersion(output: *mut c_char, output_size: c_uint) {
    on_call();
    args::write_output(output, output_size, &format!("SkyWave {VERSION}"));
}

/// # Safety
/// Called by Arma with a valid callback function.
#[no_mangle]
pub unsafe extern "C" fn RVExtensionRegisterCallback(function: callback::ArmaCallback) {
    on_call();
    callback::register(function);
}

/// # Safety
/// Called by Arma with a writable buffer and a valid function name.
#[no_mangle]
pub unsafe extern "C" fn RVExtension(output: *mut c_char, output_size: c_uint, function: *const c_char) {
    on_call();
    let name = args::read_str(function);
    let (result, _) = guarded(&name, &[]);
    args::write_output(output, output_size, &result);
}

/// # Safety
/// Called by Arma with a writable buffer, a valid function name and `argc` arguments.
#[no_mangle]
pub unsafe extern "C" fn RVExtensionArgs(
    output: *mut c_char,
    output_size: c_uint,
    function: *const c_char,
    argv: *const *const c_char,
    argc: c_uint,
) -> c_int {
    on_call();
    let name = args::read_str(function);
    let arguments = args::read_args(argv, argc);
    let (result, code) = guarded(&name, &arguments);
    args::write_output(output, output_size, &result);
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_frames() {
        let args = vec![
            "100.5,200,5,0,1,0,0,0,1".to_string(),
            "0.8".to_string(),
            "r1|110|200|5|0.7|0.6|0|120;r2|1|2|3|1|0|1|120".to_string(),
        ];
        let (listener, master, sources) = parse_frame(&args).unwrap();
        assert_eq!(listener.position, [100.5, 200.0, 5.0]);
        assert_eq!(master, 0.8);
        assert_eq!(sources.len(), 2);
        assert_eq!(sources[0].0, "r1");
        assert_eq!(sources[0].1.muffle, 0.6);
        assert!(!sources[0].1.inside && sources[1].1.inside);
    }

    #[test]
    fn accepts_empty_source_lists() {
        let args = vec!["0,0,0,0,1,0,0,0,1".to_string(), "1".to_string(), String::new()];
        assert!(parse_frame(&args).unwrap().2.is_empty());
    }

    #[test]
    fn rejects_bad_input() {
        assert!(parse_frame(&["1,2".to_string(), "1".to_string(), String::new()]).is_err());
        assert!(dispatch("nope", &[]).is_err());
        assert!(dispatch("play", &["only-id".to_string()]).is_err());
    }
}
