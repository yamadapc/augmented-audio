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
use super::power_change::PowerOfChangeFrames;

/// This is `λ(frame, bin)` in the paper.
///
/// It holds dynamic thresholds for each frequency bin. Dynamic thresholds are defined as a
/// factor of how much neighbouring frequency bins power-of-change is.
#[derive(Debug)]
pub struct DynamicThresholds {
    pub buffer: Vec<Vec<f32>>,
}

pub struct DynamicThresholdsParams {
    // This is `tau` on the paper
    pub threshold_time_spread: usize,
    // This is `beta` on the paper (Controls strength of transients to be extracted)
    pub threshold_time_spread_factor: f32,
}

/// Calculates `λ(frame, bin)`
pub fn calculate_dynamic_thresholds(
    params: DynamicThresholdsParams,
    power_of_change_frames: &PowerOfChangeFrames,
) -> DynamicThresholds {
    let DynamicThresholdsParams {
        threshold_time_spread: time_spread,
        threshold_time_spread_factor: beta,
    } = params;
    let mut result = {
        let mut result = Vec::with_capacity(power_of_change_frames.len());
        for _i in 0..power_of_change_frames.len() {
            result.push({
                let mut v = Vec::with_capacity(power_of_change_frames.bins());
                v.resize(power_of_change_frames.bins(), 0.0);
                v
            });
        }
        result
    };

    // This lint is no good
    #[allow(clippy::needless_range_loop)]
    for i in 0..power_of_change_frames.len() {
        for j in 0..power_of_change_frames.bins() {
            let mut sum = 0.0;

            {
                let i = i as i32;
                let time_spread = time_spread as i32;

                for l in i - time_spread..i + time_spread {
                    if l >= 0 && l < power_of_change_frames.len() as i32 {
                        sum += power_of_change_frames.buffer[l as usize][j];
                    }
                }
            }

            result[i][j] = beta * (sum / (2.0 * time_spread as f32 + 1.0));
        }
    }

    DynamicThresholds { buffer: result }
}
