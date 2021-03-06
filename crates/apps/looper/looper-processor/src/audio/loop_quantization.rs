// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
//! Implements quantization logic for live-loopers.
//!
//! Two different approaches are provided:
//!
//! * Snap next
//! * Snap closest
//!
//! The first is similar to most software loopers, it'll quantize by waiting until the end of the
//! current bar.
//!
//! The second is similar to hardware loopers, it'll both wait until the end of the current bar and
//! return negative offsets, in the case the button has been pressed late up to a certain threshold.

#[derive(PartialEq, Debug)]
pub enum LoopQuantizerMode {
    None,
    SnapNext { beats: usize },
    SnapClosest { beats: usize, threshold_ms: f32 },
}

pub struct QuantizeInput {
    pub tempo: f32,
    pub sample_rate: f32,
    pub position_beats: f32,
    pub position_samples: usize,
}

pub struct LoopQuantizer {
    mode: LoopQuantizerMode,
}

impl LoopQuantizer {
    pub fn new(mode: LoopQuantizerMode) -> Self {
        Self { mode }
    }

    pub fn quantize(&self, input: QuantizeInput) -> i32 {
        match &self.mode {
            LoopQuantizerMode::None => input.position_samples as i32,
            LoopQuantizerMode::SnapNext { beats } => {
                let quantized_position_beats = snap_next_beat(*beats, input.position_beats);
                Self::build_result_position_samples(&input, quantized_position_beats)
            }
            LoopQuantizerMode::SnapClosest {
                beats,
                threshold_ms,
            } => {
                let quantized_position_beats =
                    snap_closest_beat(*beats, input.tempo, *threshold_ms, input.position_beats);
                Self::build_result_position_samples(&input, quantized_position_beats)
            }
        }
    }

    fn build_result_position_samples(input: &QuantizeInput, quantized_position_beats: f32) -> i32 {
        let delta_beats = quantized_position_beats - input.position_beats;
        let secs_per_beat = 1.0 / (input.tempo / 60.0);
        let samples_per_beat = input.sample_rate * secs_per_beat;

        let position_samples = input.position_samples as i32;
        let delta_samples = (delta_beats * samples_per_beat) as i32;

        position_samples + delta_samples
    }
}

fn snap_next_beat(quantization_beats: usize, position_beats: f32) -> f32 {
    let f_beats = quantization_beats as f32;
    (position_beats / f_beats).ceil() * f_beats
}

fn snap_closest_beat(
    quantization_beats: usize,
    tempo: f32,
    threshold_ms: f32,
    position_beats: f32,
) -> f32 {
    let beats_per_ms = tempo / 60_000.0;
    let threshold_beats = beats_per_ms * threshold_ms;

    let f_beats = quantization_beats as f32;
    let ratio = position_beats / f_beats;
    let lower = ratio.floor() * f_beats;
    let upper = ratio.ceil() * f_beats;

    #[allow(clippy::float_equality_without_abs)]
    if ((lower - position_beats).abs() - threshold_beats) < f32::EPSILON {
        lower
    } else {
        upper
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_snap_next_beat() {
        let result = snap_next_beat(4, 10.1);
        assert_eq!(result as usize, 12);
        let result = snap_next_beat(4, 8.1);
        assert_eq!(result as usize, 12);
    }

    #[test]
    fn test_snap_closest_beat() {
        let result = snap_closest_beat(4, 120.0, 100.0, 8.1);
        assert_eq!(result as usize, 8);
        let result = snap_closest_beat(4, 120.0, 100.0, 10.1);
        assert_eq!(result as usize, 12);
        let result = snap_closest_beat(4, 120.0, 100.0, 11.1);
        assert_eq!(result as usize, 12);
    }

    #[test]
    fn test_quantization_none() {
        let quantizer = LoopQuantizer::new(LoopQuantizerMode::None);
        let result = quantizer.quantize(QuantizeInput {
            tempo: 120.0,
            sample_rate: 1000.0,
            position_beats: 10.0,
            position_samples: 0,
        });
        assert_eq!(result, 0);
    }

    #[test]
    fn test_quantization_snap_next() {
        let quantizer = LoopQuantizer::new(LoopQuantizerMode::SnapNext { beats: 4 });
        let result = quantizer.quantize(QuantizeInput {
            tempo: 60.0, // 1 beat per s ; 1000 samples per beat ; we should wait 2000 smpl
            sample_rate: 1000.0,
            position_beats: 10.0, // Should wait until beat 12
            position_samples: 5000,
        });
        assert_eq!(result, 7000);
    }

    #[test]
    fn test_quantization_snap_closest() {
        let quantizer = LoopQuantizer::new(LoopQuantizerMode::SnapClosest {
            beats: 4,
            threshold_ms: 200.0,
        });

        let result = quantizer.quantize(QuantizeInput {
            tempo: 60.0,
            sample_rate: 1000.0,
            position_beats: 8.1,
            position_samples: 1000,
        });
        assert_eq!(result, 900);

        let result = quantizer.quantize(QuantizeInput {
            tempo: 60.0,
            sample_rate: 1000.0,
            position_beats: 11.9,
            position_samples: 5000,
        });
        assert_eq!(result, 5100);
    }
}
