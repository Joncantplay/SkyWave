// Copyright (C) 2026 Joncantplay
// SPDX-License-Identifier: GPL-3.0-or-later
//! Converts decoded audio to stereo at the output device's sample rate.

/// Streaming stereo resampler with cubic (Catmull-Rom) interpolation.
pub struct Resampler {
    in_rate: u32,
    out_rate: u32,
    step: f64,
    position: f64,
    history: Vec<[f32; 2]>,
}

impl Resampler {
    pub fn new(in_rate: u32, out_rate: u32) -> Resampler {
        Resampler {
            in_rate,
            out_rate,
            step: in_rate as f64 / out_rate as f64,
            position: 1.0,
            history: vec![[0.0; 2]],
        }
    }

    pub fn rates(&self) -> (u32, u32) {
        (self.in_rate, self.out_rate)
    }

    /// Appends resampled interleaved stereo samples for `input` to `out`.
    pub fn process(&mut self, input: &[[f32; 2]], out: &mut Vec<f32>) {
        self.history.extend_from_slice(input);
        loop {
            let i = self.position as usize;
            if i + 2 >= self.history.len() {
                break;
            }
            let t = (self.position - i as f64) as f32;
            let h = &self.history;
            for c in 0..2 {
                out.push(catmull_rom(h[i - 1][c], h[i][c], h[i + 1][c], h[i + 2][c], t));
            }
            self.position += self.step;
        }
        let consumed = (self.position as usize).saturating_sub(1).min(self.history.len());
        self.history.drain(..consumed);
        self.position -= consumed as f64;
    }
}

fn catmull_rom(p0: f32, p1: f32, p2: f32, p3: f32, t: f32) -> f32 {
    0.5 * (2.0 * p1
        + (-p0 + p2) * t
        + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t * t
        + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t * t * t)
}

/// Converts interleaved samples with any channel count to stereo frames.
pub fn to_stereo(samples: &[f32], channels: usize, out: &mut Vec<[f32; 2]>) {
    out.clear();
    match channels {
        0 => {}
        1 => out.extend(samples.iter().map(|&s| [s, s])),
        n => out.extend(samples.chunks_exact(n).map(|f| [f[0], f[1]])),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_rate_passes_samples_through() {
        let mut r = Resampler::new(48000, 48000);
        let input: Vec<[f32; 2]> = (0..100).map(|i| [i as f32, -(i as f32)]).collect();
        let mut out = Vec::new();
        r.process(&input[..50], &mut out);
        r.process(&input[50..], &mut out);
        let left: Vec<f32> = out.chunks(2).map(|f| f[0]).collect();
        assert!(left.len() >= 97);
        for (i, v) in left.iter().enumerate() {
            assert!((v - i as f32).abs() < 1e-4, "sample {i} was {v}");
        }
    }

    #[test]
    fn output_length_follows_rate_ratio() {
        let mut r = Resampler::new(44100, 48000);
        let input = vec![[0.25f32, 0.25]; 44100];
        let mut out = Vec::new();
        for chunk in input.chunks(1000) {
            r.process(chunk, &mut out);
        }
        let frames = out.len() / 2;
        assert!((47990..=48000).contains(&frames), "got {frames} frames");
        assert!(out[100..].iter().all(|v| (v - 0.25).abs() < 1e-4));
    }

    #[test]
    fn converts_channel_layouts() {
        let mut out = Vec::new();
        to_stereo(&[0.1, 0.2], 1, &mut out);
        assert_eq!(out, vec![[0.1, 0.1], [0.2, 0.2]]);
        to_stereo(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], 3, &mut out);
        assert_eq!(out, vec![[1.0, 2.0], [4.0, 5.0]]);
    }
}
