// Copyright (C) 2026 Joncantplay
// SPDX-License-Identifier: GPL-3.0-or-later
//! Copies text to the Windows clipboard. The game's own command for this is disabled in
//! multiplayer, so the radio window uses the extension to copy the current track title.

use std::ffi::c_void;

const CF_UNICODETEXT: u32 = 13;
const GMEM_MOVEABLE: u32 = 0x0002;

#[link(name = "user32")]
extern "system" {
    fn OpenClipboard(owner: *mut c_void) -> i32;
    fn EmptyClipboard() -> i32;
    fn SetClipboardData(format: u32, memory: *mut c_void) -> *mut c_void;
    fn CloseClipboard() -> i32;
}

#[link(name = "kernel32")]
extern "system" {
    fn GlobalAlloc(flags: u32, bytes: usize) -> *mut c_void;
    fn GlobalLock(memory: *mut c_void) -> *mut c_void;
    fn GlobalUnlock(memory: *mut c_void) -> i32;
    fn GlobalFree(memory: *mut c_void) -> *mut c_void;
}

/// UTF-16 with a terminating zero, as the clipboard expects.
pub fn to_wide(text: &str) -> Vec<u16> {
    text.encode_utf16().chain(std::iter::once(0)).collect()
}

pub fn copy(text: &str) -> Result<(), String> {
    let wide = to_wide(text);
    let bytes = wide.len() * 2;
    unsafe {
        if OpenClipboard(std::ptr::null_mut()) == 0 {
            return Err("clipboard is busy".to_string());
        }
        let result = (|| {
            EmptyClipboard();
            let memory = GlobalAlloc(GMEM_MOVEABLE, bytes);
            if memory.is_null() {
                return Err("out of memory".to_string());
            }
            let target = GlobalLock(memory) as *mut u16;
            if target.is_null() {
                GlobalFree(memory);
                return Err("could not lock clipboard memory".to_string());
            }
            std::ptr::copy_nonoverlapping(wide.as_ptr(), target, wide.len());
            GlobalUnlock(memory);
            // On success the clipboard owns the memory.
            if SetClipboardData(CF_UNICODETEXT, memory).is_null() {
                GlobalFree(memory);
                return Err("could not set clipboard data".to_string());
            }
            Ok(())
        })();
        CloseClipboard();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_wide_text() {
        assert_eq!(to_wide("Aä"), vec![0x41, 0xE4, 0]);
        assert_eq!(to_wide(""), vec![0]);
    }
}
