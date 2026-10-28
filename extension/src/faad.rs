// Copyright (C) 2026 Joncantplay
// SPDX-License-Identifier: GPL-3.0-or-later
//! Bindings to FAAD2, the AAC / HE-AAC (AAC+) decoder.
//!
//! Code from FAAD2 is copyright (c) Nero AG, www.nero.com

#![allow(non_snake_case)]

use std::ffi::{c_char, c_long, c_uchar, c_ulong, c_void, CStr};

#[repr(C)]
struct NeAACDecConfiguration {
    defObjectType: c_uchar,
    defSampleRate: c_ulong,
    outputFormat: c_uchar,
    downMatrix: c_uchar,
    useOldADTSFormat: c_uchar,
    dontUpSampleImplicitSBR: c_uchar,
}

#[repr(C)]
#[allow(dead_code)]
struct NeAACDecFrameInfo {
    bytesconsumed: c_ulong,
    samples: c_ulong,
    channels: c_uchar,
    error: c_uchar,
    samplerate: c_ulong,
    sbr: c_uchar,
    object_type: c_uchar,
    header_type: c_uchar,
    num_front_channels: c_uchar,
    num_side_channels: c_uchar,
    num_back_channels: c_uchar,
    num_lfe_channels: c_uchar,
    channel_position: [c_uchar; 64],
    ps: c_uchar,
}

extern "C" {
    fn NeAACDecOpen() -> *mut c_void;
    fn NeAACDecGetCurrentConfiguration(decoder: *mut c_void) -> *mut NeAACDecConfiguration;
    fn NeAACDecSetConfiguration(decoder: *mut c_void, config: *mut NeAACDecConfiguration) -> c_uchar;
    fn NeAACDecInit(
        decoder: *mut c_void,
        buffer: *mut c_uchar,
        buffer_size: c_ulong,
        samplerate: *mut c_ulong,
        channels: *mut c_uchar,
    ) -> c_long;
    fn NeAACDecDecode(
        decoder: *mut c_void,
        info: *mut NeAACDecFrameInfo,
        buffer: *mut c_uchar,
        buffer_size: c_ulong,
    ) -> *mut c_void;
    fn NeAACDecGetErrorMessage(code: c_uchar) -> *mut c_char;
    fn NeAACDecClose(decoder: *mut c_void);
}

const OBJECT_TYPE_LC: c_uchar = 2;
const FORMAT_FLOAT: c_uchar = 4;

pub struct Frame<'a> {
    pub consumed: usize,
    /// Interleaved samples in the range -1..1.
    pub samples: &'a [f32],
    pub channels: usize,
    pub rate: u32,
    pub error: Option<String>,
}

pub struct AacDecoder {
    handle: *mut c_void,
}

impl AacDecoder {
    pub fn new() -> Option<AacDecoder> {
        unsafe {
            let handle = NeAACDecOpen();
            if handle.is_null() {
                return None;
            }
            let config = NeAACDecGetCurrentConfiguration(handle);
            if !config.is_null() {
                (*config).defObjectType = OBJECT_TYPE_LC;
                (*config).outputFormat = FORMAT_FLOAT;
                // Mix multichannel streams down to stereo.
                (*config).downMatrix = 1;
                NeAACDecSetConfiguration(handle, config);
            }
            Some(AacDecoder { handle })
        }
    }

    /// Prepares the decoder from ADTS data. Returns the number of bytes to skip.
    pub fn init_adts(&mut self, data: &mut [u8]) -> Result<usize, String> {
        let mut rate: c_ulong = 0;
        let mut channels: c_uchar = 0;
        let result = unsafe { NeAACDecInit(self.handle, data.as_mut_ptr(), data.len() as c_ulong, &mut rate, &mut channels) };
        if result < 0 {
            Err("AAC decoder could not read the stream header".to_string())
        } else {
            Ok(result as usize)
        }
    }

    pub fn decode(&mut self, data: &mut [u8]) -> Frame<'_> {
        let mut info: NeAACDecFrameInfo = unsafe { std::mem::zeroed() };
        let output = unsafe { NeAACDecDecode(self.handle, &mut info, data.as_mut_ptr(), data.len() as c_ulong) };
        let error = (info.error != 0).then(|| unsafe {
            let message = NeAACDecGetErrorMessage(info.error);
            if message.is_null() {
                format!("error {}", info.error)
            } else {
                CStr::from_ptr(message).to_string_lossy().into_owned()
            }
        });
        let samples = if output.is_null() || error.is_some() || info.samples == 0 {
            &[][..]
        } else {
            unsafe { std::slice::from_raw_parts(output as *const f32, info.samples as usize) }
        };
        Frame { consumed: info.bytesconsumed as usize, samples, channels: info.channels as usize, rate: info.samplerate as u32, error }
    }
}

impl Drop for AacDecoder {
    fn drop(&mut self) {
        unsafe { NeAACDecClose(self.handle) };
    }
}
