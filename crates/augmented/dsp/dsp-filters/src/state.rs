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
use num::Float;

use crate::coefficients::BiquadCoefficients;

pub trait FilterState {
    type Sample: Float;

    fn reset(&mut self);
    fn process1(
        &mut self,
        coefficients: &BiquadCoefficients<Self::Sample>,
        input: Self::Sample,
        very_small_amount: Self::Sample,
    ) -> Self::Sample;
}

/// State for applying a second order section to a sample using Direct Form I
/// Difference equation:
///
/// ```ignore
/// y[n] = (b0/a0)*x[n] + (b1/a0)*x[n-1] + (b2/a0)*x[n-2]
///                     - (a1/a0)*y[n-1] - (a2/a0)*y[n-2]
/// ```
pub struct DirectFormIState<Sample: Float> {
    x2: Sample, // x[n - 2]
    y2: Sample, // y[n - 2]
    x1: Sample, // x[n - 1]
    y1: Sample, // y[n - 1]
}

impl<Sample: Float> Default for DirectFormIState<Sample> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Sample: Float> DirectFormIState<Sample> {
    pub fn new() -> Self {
        DirectFormIState {
            x2: Sample::zero(),
            y2: Sample::zero(),
            x1: Sample::zero(),
            y1: Sample::zero(),
        }
    }
}

impl<Sample: Float> FilterState for DirectFormIState<Sample> {
    type Sample = Sample;

    fn reset(&mut self) {
        self.x1 = Sample::zero();
        self.x2 = Sample::zero();
        self.y1 = Sample::zero();
        self.y2 = Sample::zero();
    }

    fn process1(
        &mut self,
        coefficients: &BiquadCoefficients<Sample>,
        input: Sample,
        very_small_amount: Sample,
    ) -> Sample {
        let output = {
            let BiquadCoefficients {
                b0, b1, b2, a1, a2, ..
            } = *coefficients;
            let DirectFormIState { x1, x2, y2, y1 } = *self;
            b0 * input + b1 * x1 + b2 * x2 - a1 * y1 - a2 * y2 + very_small_amount
        };
        self.x2 = self.x1;
        self.y2 = self.y1;
        self.x1 = input;
        self.y1 = output;
        assert!(!self.y1.is_nan());
        assert!(!self.y2.is_nan());

        output
    }
}

/// State for applying a second order section to a sample using Direct Form II
///
/// Difference equation:
///
/// ```ignore
/// v[n] =         x[n] - (a1/a0)*v[n-1] - (a2/a0)*v[n-2]
/// y(n) = (b0/a0)*v[n] + (b1/a0)*v[n-1] + (b2/a0)*v[n-2]
/// ```
pub struct DirectFormIIState<Sample: Float> {
    v1: Sample, // v[-1]
    v2: Sample, // v[-2]
}

impl<Sample: Float> FilterState for DirectFormIIState<Sample> {
    type Sample = Sample;

    fn reset(&mut self) {
        self.v1 = Sample::zero();
        self.v2 = Sample::zero();
    }

    fn process1(
        &mut self,
        coefficients: &BiquadCoefficients<Sample>,
        input: Sample,
        very_small_amount: Sample,
    ) -> Sample {
        let BiquadCoefficients {
            a1, a2, b0, b1, b2, ..
        } = *coefficients;
        let DirectFormIIState { v1, v2 } = *self;
        let w = input - a1 * v1 - a2 * v2 + very_small_amount;
        let output = b0 * w + b1 * v1 + b2 * v2;

        self.v2 = v1;
        self.v1 = w;

        output
    }
}
