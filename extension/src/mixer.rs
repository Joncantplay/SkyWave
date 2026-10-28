// Copyright (C) 2026 Joncantplay
// SPDX-License-Identifier: GPL-3.0-or-later
//! 3D mixer: places every radio relative to the listener and mixes them to stereo.
//!
//! Coordinates are Arma world space (x east, y north, z up, metres).

use std::collections::HashMap;
use std::f32::consts::{FRAC_PI_4, SQRT_2, TAU};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use crate::stream::SourceBuffer;

/// Distance up to which a radio plays at full volume.
const REFERENCE_DISTANCE: f32 = 2.0;
/// How quickly the volume falls off beyond the reference distance.
const ROLLOFF: f32 = 0.5;
/// Cutoff frequency for a fully muffled radio (closed vehicle, both sides).
const MUFFLED_CUTOFF: f32 = 600.0;
const OPEN_CUTOFF: f32 = 18000.0;
/// Air absorption: the cutoff halves every this many metres (roughly).
const AIR_DISTANCE: f32 = 40.0;
/// Without position updates from the game for this long, everything fades out.
const STALE_AFTER: Duration = Duration::from_secs(1);

pub type Vec3 = [f32; 3];

#[derive(Clone, Copy, Debug)]
pub struct Listener {
    pub position: Vec3,
    pub forward: Vec3,
    pub up: Vec3,
}

#[derive(Clone, Copy, Debug)]
pub struct SourceParams {
    pub position: Vec3,
    /// Radio volume 0..1 as set by the players.
    pub volume: f32,
    /// 0 = nothing between radio and listener, 1 = fully muffled.
    pub muffle: f32,
    /// Listener sits in the vehicle that carries this radio.
    pub inside: bool,
    /// Audible range in metres; the radio fades out towards it.
    pub range: f32,
}

/// What a source should sound like right now.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Targets {
    pub left: f32,
    pub right: f32,
    /// 0 = keep stereo, 1 = mono point source.
    pub mono: f32,
    pub cutoff: f32,
}

struct MixSource {
    buffer: Arc<SourceBuffer>,
    params: Option<SourceParams>,
    current: Targets,
    filter: [[f32; 2]; 2],
}

pub struct Mixer {
    sources: HashMap<String, MixSource>,
    listener: Listener,
    master: f32,
    last_update: Option<Instant>,
    scratch: Vec<f32>,
}

static MIXER: Mutex<Option<Mixer>> = Mutex::new(None);

fn mixer() -> MutexGuard<'static, Option<Mixer>> {
    let mut guard = MIXER.lock().unwrap_or_else(|e| e.into_inner());
    if guard.is_none() {
        *guard = Some(Mixer {
            sources: HashMap::new(),
            listener: Listener { position: [0.0; 3], forward: [0.0, 1.0, 0.0], up: [0.0, 0.0, 1.0] },
            master: 0.0,
            last_update: None,
            scratch: Vec::new(),
        });
    }
    guard
}

const SILENT: Targets = Targets { left: 0.0, right: 0.0, mono: 1.0, cutoff: OPEN_CUTOFF };

pub fn add_source(id: &str, buffer: Arc<SourceBuffer>) {
    if let Some(m) = mixer().as_mut() {
        m.sources.insert(id.to_string(), MixSource { buffer, params: None, current: SILENT, filter: [[0.0; 2]; 2] });
    }
}

pub fn remove_source(id: &str) {
    if let Some(m) = mixer().as_mut() {
        m.sources.remove(id);
    }
}

pub fn clear() {
    if let Some(m) = mixer().as_mut() {
        m.sources.clear();
    }
}

pub fn update(listener: Listener, master: f32, params: Vec<(String, SourceParams)>) {
    if let Some(m) = mixer().as_mut() {
        m.listener = listener;
        m.master = master.clamp(0.0, MAX_MASTER);
        m.last_update = Some(Instant::now());
        for source in m.sources.values_mut() {
            source.params = None;
        }
        for (id, p) in params {
            if let Some(source) = m.sources.get_mut(&id) {
                source.params = Some(p);
            }
        }
    }
}

/// Seconds since the game last sent positions (large when it never did).
pub fn seconds_since_update() -> f32 {
    mixer().as_ref().and_then(|m| m.last_update).map_or(f32::MAX, |t| t.elapsed().as_secs_f32())
}

fn sub(a: Vec3, b: Vec3) -> Vec3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn dot(a: Vec3, b: Vec3) -> f32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: Vec3, b: Vec3) -> Vec3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn normalize(v: Vec3) -> Option<Vec3> {
    let length = dot(v, v).sqrt();
    (length > 1e-5).then(|| [v[0] / length, v[1] / length, v[2] / length])
}

/// Computes gains and filtering for one radio as heard by the listener.
pub fn targets(p: &SourceParams, l: &Listener) -> Targets {
    let volume = p.volume.clamp(0.0, 1.0);
    // Squared so the volume slider feels even to the ear.
    let gain = volume * volume;
    if p.inside {
        return Targets { left: gain, right: gain, mono: 0.0, cutoff: OPEN_CUTOFF };
    }

    let offset = sub(p.position, l.position);
    let distance = dot(offset, offset).sqrt();
    let attenuation = REFERENCE_DISTANCE / (REFERENCE_DISTANCE + ROLLOFF * (distance - REFERENCE_DISTANCE).max(0.0));
    let fade = if p.range > 0.0 { ((p.range - distance) / (p.range * 0.25)).clamp(0.0, 1.0) } else { 1.0 };

    let forward = normalize(l.forward).unwrap_or([0.0, 1.0, 0.0]);
    let right = normalize(cross(forward, l.up)).or_else(|| normalize(cross(forward, [0.0, 0.0, 1.0]))).unwrap_or([1.0, 0.0, 0.0]);
    let direction = normalize(offset).unwrap_or(forward);
    let pan = dot(direction, right).clamp(-1.0, 1.0);
    let front = dot(direction, forward);
    let rear = if front < 0.0 { 1.0 + 0.3 * front } else { 1.0 };

    let muffle = p.muffle.clamp(0.0, 1.0);
    let level = gain * attenuation * fade * rear * (1.0 - 0.5 * muffle);
    let angle = (pan + 1.0) * FRAC_PI_4;
    let muffle_cutoff = OPEN_CUTOFF * (MUFFLED_CUTOFF / OPEN_CUTOFF).powf(muffle);
    let air_cutoff = OPEN_CUTOFF / (1.0 + distance / AIR_DISTANCE);

    Targets {
        left: level * angle.cos() * SQRT_2,
        right: level * angle.sin() * SQRT_2,
        mono: 1.0,
        cutoff: muffle_cutoff.min(air_cutoff),
    }
}

fn filter_coefficient(cutoff: f32, rate: f32) -> f32 {
    if cutoff >= OPEN_CUTOFF {
        1.0
    } else {
        1.0 - (-TAU * cutoff / rate).exp()
    }
}

/// Mixes all sources into `out` (interleaved stereo). Called from the audio thread.
pub fn render(out: &mut [f32], rate: u32) {
    out.fill(0.0);
    let mut guard = mixer();
    let Some(m) = guard.as_mut() else { return };
    let frames = out.len() / 2;
    if frames == 0 {
        return;
    }
    let stale = m.last_update.map_or(true, |t| t.elapsed() > STALE_AFTER);
    let master = if stale { 0.0 } else { m.master };
    let listener = m.listener;
    let rate_f = rate as f32;
    m.scratch.resize(frames * 2, 0.0);

    for source in m.sources.values_mut() {
        let mut target = match (&source.params, stale) {
            (Some(p), false) => targets(p, &listener),
            _ => Targets { mono: source.current.mono, ..SILENT },
        };
        target.left *= master;
        target.right *= master;

        // Always consume audio so a muted radio stays in sync with the live stream.
        let written = source.buffer.pop_into(&mut m.scratch, rate);
        m.scratch[written..].fill(0.0);

        let from = source.current;
        let a_from = filter_coefficient(from.cutoff, rate_f);
        let a_to = filter_coefficient(target.cutoff, rate_f);
        let step = 1.0 / frames as f32;
        let silent = from.left == 0.0 && from.right == 0.0 && target.left == 0.0 && target.right == 0.0;
        if !silent {
            for k in 0..frames {
                let t = (k + 1) as f32 * step;
                let gl = from.left + (target.left - from.left) * t;
                let gr = from.right + (target.right - from.right) * t;
                let mono = from.mono + (target.mono - from.mono) * t;
                let a = a_from + (a_to - a_from) * t;
                let (l, r) = (m.scratch[2 * k], m.scratch[2 * k + 1]);
                let mid = 0.5 * (l + r);
                let input = [l + (mid - l) * mono, r + (mid - r) * mono];
                for c in 0..2 {
                    let stage1 = &mut source.filter[0][c];
                    *stage1 += a * (input[c] - *stage1);
                    let s1 = *stage1;
                    let stage2 = &mut source.filter[1][c];
                    *stage2 += a * (s1 - *stage2);
                }
                out[2 * k] += source.filter[1][0] * gl;
                out[2 * k + 1] += source.filter[1][1] * gr;
            }
        }
        source.current = target;
    }

    for sample in out.iter_mut() {
        *sample = limit(*sample);
    }
}

/// Highest master volume: the player's volume setting times the boost setting.
const MAX_MASTER: f32 = 4.0;
/// Level above which the limiter starts to round off peaks.
const LIMIT_KNEE: f32 = 0.8;

/// Soft limiter: leaves quiet samples alone and rounds peaks off towards 1.0 instead of
/// clipping them hard (matters with the volume boost).
fn limit(sample: f32) -> f32 {
    let level = sample.abs();
    if level <= LIMIT_KNEE {
        return sample;
    }
    let room = 1.0 - LIMIT_KNEE;
    (LIMIT_KNEE + room * ((level - LIMIT_KNEE) / room).tanh()).copysign(sample)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn listener() -> Listener {
        // Standing at the origin, looking north.
        Listener { position: [0.0; 3], forward: [0.0, 1.0, 0.0], up: [0.0, 0.0, 1.0] }
    }

    fn params(position: Vec3) -> SourceParams {
        SourceParams { position, volume: 1.0, muffle: 0.0, inside: false, range: 120.0 }
    }

    #[test]
    fn pans_to_the_correct_side() {
        let east = targets(&params([10.0, 0.0, 0.0]), &listener());
        assert!(east.right > east.left * 10.0, "{east:?}");
        let west = targets(&params([-10.0, 0.0, 0.0]), &listener());
        assert!(west.left > west.right * 10.0, "{west:?}");
        let ahead = targets(&params([0.0, 10.0, 0.0]), &listener());
        assert!((ahead.left - ahead.right).abs() < 1e-4);
    }

    #[test]
    fn gets_quieter_and_duller_with_distance() {
        let near = targets(&params([0.0, 5.0, 0.0]), &listener());
        let far = targets(&params([0.0, 60.0, 0.0]), &listener());
        assert!(far.left < near.left);
        assert!(far.cutoff < near.cutoff);
        let outside = targets(&params([0.0, 130.0, 0.0]), &listener());
        assert_eq!(outside.left, 0.0);
    }

    #[test]
    fn inside_keeps_stereo_and_full_volume() {
        let mut p = params([50.0, 50.0, 0.0]);
        p.inside = true;
        p.volume = 0.5;
        let t = targets(&p, &listener());
        assert_eq!(t, Targets { left: 0.25, right: 0.25, mono: 0.0, cutoff: OPEN_CUTOFF });
    }

    #[test]
    fn limiter_keeps_quiet_samples_and_caps_loud_ones() {
        assert_eq!(limit(0.5), 0.5);
        assert_eq!(limit(-0.8), -0.8);
        assert!(limit(0.9) > 0.8 && limit(0.9) < 0.9);
        assert!(limit(4.0) <= 1.0 && limit(4.0) > 0.99);
        assert!(limit(-4.0) >= -1.0 && limit(-4.0) < -0.99);
        assert!(limit(1.2) > limit(1.0));
    }

    #[test]
    fn muffling_lowers_the_cutoff() {
        let mut p = params([0.0, 3.0, 0.0]);
        let open = targets(&p, &listener());
        p.muffle = 0.6;
        let closed = targets(&p, &listener());
        assert!(closed.cutoff < 3000.0 && open.cutoff > 10000.0, "{open:?} {closed:?}");
        assert!(closed.left < open.left);
    }
}
