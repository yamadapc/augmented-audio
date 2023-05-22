use rustfft::num_traits::Float;

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

#[numeric_literals::replace_float_literals(F::from(literal).unwrap())]
pub fn calculate_multiplier<F>(sample_rate: F, duration_ms: F) -> F
where
    F: Float,
{
    let attack_secs = duration_ms * 0.001;
    let attack_samples = sample_rate * attack_secs;
    F::exp2(-1.0 / attack_samples)
}

pub type PeakDetector = PeakDetectorImpl<f32>;

pub struct PeakDetectorImpl<F> {
    value: F,
}

impl<F: Float> Default for PeakDetectorImpl<F> {
    #[numeric_literals::replace_float_literals(F::from(literal).unwrap())]
    fn default() -> Self {
        Self { value: 0.0 }
    }
}

impl<F: Float + std::iter::Sum> PeakDetectorImpl<F> {
    pub fn value(&self) -> F {
        self.value
    }

    #[numeric_literals::replace_float_literals(F::from(literal).unwrap())]
    pub fn accept_frame(&mut self, attack_mult: F, release_mult: F, frame: &[F]) {
        let frame_len = F::from(frame.len()).unwrap();
        let new: F = frame.iter().map(|f| F::abs(*f)).sum::<F>() / frame_len;
        let curr_slope = if self.value > new {
            release_mult
        } else {
            attack_mult
        };
        self.value = (self.value * curr_slope) + ((1.0 - curr_slope) * new);
    }
}
