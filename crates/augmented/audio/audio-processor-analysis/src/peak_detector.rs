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
type FloatT = f32;

pub fn calculate_multiplier(sample_rate: FloatT, duration_ms: FloatT) -> FloatT {
    let attack_secs = duration_ms * 0.001;
    let attack_samples = sample_rate * attack_secs;
    FloatT::exp2(-1.0 / attack_samples)
}

pub struct PeakDetector {
    value: FloatT,
}

impl Default for PeakDetector {
    fn default() -> Self {
        Self { value: 0.0 }
    }
}

impl PeakDetector {
    pub fn value(&self) -> FloatT {
        self.value
    }

    pub fn accept_frame(&mut self, attack_mult: FloatT, release_mult: FloatT, frame: &[FloatT]) {
        let frame_len = frame.len() as FloatT;
        let new: FloatT = frame.iter().map(|f| FloatT::abs(*f)).sum::<FloatT>() / frame_len;
        let curr_slope = if self.value > new {
            release_mult
        } else {
            attack_mult
        };
        self.value = (self.value * curr_slope) + ((1.0 - curr_slope) * new);
    }
}
