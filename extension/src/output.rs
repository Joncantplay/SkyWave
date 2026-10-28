// Copyright (C) 2026 Joncantplay
// SPDX-License-Identifier: GPL-3.0-or-later
//! Sound card output. Runs on its own thread and reopens the device when it disappears
//! or when Windows switches the default device (e.g. headset plugged in).

use std::panic::AssertUnwindSafe;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Once};
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::callback;
use crate::mixer;

static DEVICE_RATE: AtomicU32 = AtomicU32::new(48_000);
static START: Once = Once::new();

/// Sample rate of the current output device.
pub fn device_rate() -> u32 {
    DEVICE_RATE.load(Ordering::Relaxed)
}

pub fn start() {
    START.call_once(|| {
        let spawned = std::thread::Builder::new().name("skywave-output".into()).spawn(run);
        if let Err(e) = spawned {
            callback::log(format!("could not start audio output thread: {e}"));
        }
    });
}

fn run() {
    let mut last_error = String::new();
    loop {
        match open() {
            Ok((stream, name, failed)) => {
                callback::log(format!("audio output: {name} at {} Hz", device_rate()));
                last_error.clear();
                let mut ticks = 0u32;
                loop {
                    std::thread::sleep(Duration::from_millis(500));
                    if failed.load(Ordering::SeqCst) {
                        break;
                    }
                    ticks += 1;
                    if ticks % 4 == 0 && default_device_name().is_some_and(|current| current != name) {
                        callback::log("default audio device changed");
                        break;
                    }
                }
                drop(stream);
            }
            Err(e) => {
                if e != last_error {
                    callback::log(format!("audio output unavailable: {e}"));
                    last_error = e;
                }
                std::thread::sleep(Duration::from_secs(3));
            }
        }
    }
}

fn default_device_name() -> Option<String> {
    cpal::default_host().default_output_device().and_then(|d| d.name().ok())
}

fn open() -> Result<(cpal::Stream, String, Arc<AtomicBool>), String> {
    let host = cpal::default_host();
    let device = host.default_output_device().ok_or("no audio output device")?;
    let name = device.name().unwrap_or_else(|_| "unknown device".to_string());
    let supported = device.default_output_config().map_err(|e| e.to_string())?;
    let format = supported.sample_format();
    let config: cpal::StreamConfig = supported.config();
    DEVICE_RATE.store(config.sample_rate.0, Ordering::Relaxed);

    let failed = Arc::new(AtomicBool::new(false));
    let stream = match format {
        cpal::SampleFormat::F32 => build::<f32>(&device, &config, failed.clone()),
        cpal::SampleFormat::I16 => build::<i16>(&device, &config, failed.clone()),
        cpal::SampleFormat::U16 => build::<u16>(&device, &config, failed.clone()),
        cpal::SampleFormat::I32 => build::<i32>(&device, &config, failed.clone()),
        other => Err(format!("unsupported sample format {other:?}")),
    }?;
    stream.play().map_err(|e| e.to_string())?;
    Ok((stream, name, failed))
}

fn build<T>(device: &cpal::Device, config: &cpal::StreamConfig, failed: Arc<AtomicBool>) -> Result<cpal::Stream, String>
where
    T: cpal::SizedSample + cpal::FromSample<f32>,
{
    let channels = config.channels as usize;
    let rate = config.sample_rate.0;
    let mut mix: Vec<f32> = Vec::new();
    device
        .build_output_stream(
            config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                let frames = data.len() / channels.max(1);
                mix.resize(frames * 2, 0.0);
                if std::panic::catch_unwind(AssertUnwindSafe(|| mixer::render(&mut mix, rate))).is_err() {
                    mix.fill(0.0);
                }
                for (i, frame) in data.chunks_mut(channels.max(1)).enumerate() {
                    let (l, r) = (mix[2 * i], mix[2 * i + 1]);
                    if frame.len() == 1 {
                        frame[0] = T::from_sample(0.5 * (l + r));
                        continue;
                    }
                    frame[0] = T::from_sample(l);
                    frame[1] = T::from_sample(r);
                    for extra in &mut frame[2..] {
                        *extra = T::from_sample(0.0f32);
                    }
                }
            },
            move |error| {
                callback::log(format!("audio output error: {error}"));
                failed.store(true, Ordering::SeqCst);
            },
            None,
        )
        .map_err(|e| e.to_string())
}
