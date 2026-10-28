// Copyright (C) 2026 Joncantplay
// SPDX-License-Identifier: GPL-3.0-or-later
//! Conversion between Arma's extension interface and Rust strings.

use std::ffi::{c_char, CStr};

/// Turns one argument as Arma passes it into plain text.
///
/// `callExtension` converts every argument with `str`, so strings arrive wrapped in
/// quotes with inner quotes doubled.
pub fn unquote(raw: &str) -> String {
    let t = raw.trim();
    if t.len() >= 2 && t.starts_with('"') && t.ends_with('"') {
        t[1..t.len() - 1].replace("\"\"", "\"")
    } else {
        t.to_string()
    }
}

/// Reads the argument array of `RVExtensionArgs`.
///
/// # Safety
/// `argv` must point to `argc` valid C strings (or be null).
pub unsafe fn read_args(argv: *const *const c_char, argc: u32) -> Vec<String> {
    if argv.is_null() {
        return Vec::new();
    }
    (0..argc as usize)
        .map(|i| {
            let p = *argv.add(i);
            if p.is_null() {
                String::new()
            } else {
                unquote(&CStr::from_ptr(p).to_string_lossy())
            }
        })
        .collect()
}

/// Reads a C string passed by Arma.
///
/// # Safety
/// `p` must be a valid C string or null.
pub unsafe fn read_str(p: *const c_char) -> String {
    if p.is_null() {
        String::new()
    } else {
        CStr::from_ptr(p).to_string_lossy().into_owned()
    }
}

/// Copies `text` into Arma's output buffer, truncated to fit and null-terminated.
///
/// # Safety
/// `output` must point to a writable buffer of `size` bytes.
pub unsafe fn write_output(output: *mut c_char, size: u32, text: &str) {
    if output.is_null() || size == 0 {
        return;
    }
    let mut end = text.len().min(size as usize - 1);
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    std::ptr::copy_nonoverlapping(text.as_ptr() as *const c_char, output, end);
    *output.add(end) = 0;
}

/// Formats text as an SQF string literal that `parseSimpleArray` understands.
pub fn sqf_string(s: &str) -> String {
    let clean: String = s.chars().map(|c| if c.is_control() { ' ' } else { c }).collect();
    format!("\"{}\"", clean.replace('"', "\"\""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unquote_strips_sqf_quoting() {
        assert_eq!(unquote("\"r1\""), "r1");
        assert_eq!(unquote("\"say \"\"hi\"\"\""), "say \"hi\"");
        assert_eq!(unquote("0.5"), "0.5");
        assert_eq!(unquote("\"\""), "");
    }

    #[test]
    fn sqf_string_escapes_quotes_and_controls() {
        assert_eq!(sqf_string("a\"b"), "\"a\"\"b\"");
        assert_eq!(sqf_string("line\nbreak"), "\"line break\"");
    }

    #[test]
    fn write_output_truncates_on_char_boundary() {
        let mut buf = [1 as c_char; 4];
        unsafe { write_output(buf.as_mut_ptr(), 4, "aä€") };
        let s = unsafe { CStr::from_ptr(buf.as_ptr()) }.to_str().unwrap();
        assert_eq!(s, "aä");
    }
}
