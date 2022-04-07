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

pub struct BiquadCoefficients<Sample: Float> {
    pub(crate) a0: Sample,
    pub(crate) a1: Sample,
    pub(crate) a2: Sample,
    pub(crate) b1: Sample,
    pub(crate) b2: Sample,
    pub(crate) b0: Sample,
}

impl<Sample: Float> Default for BiquadCoefficients<Sample> {
    fn default() -> Self {
        BiquadCoefficients {
            a0: Sample::zero(),
            a1: Sample::zero(),
            a2: Sample::zero(),
            b1: Sample::zero(),
            b2: Sample::zero(),
            b0: Sample::zero(),
        }
    }
}

impl<Sample: Float> BiquadCoefficients<Sample> {
    pub fn set_coefficients(
        &mut self,
        a0: Sample,
        a1: Sample,
        a2: Sample,
        b0: Sample,
        b1: Sample,
        b2: Sample,
    ) {
        assert!(
            !a0.is_nan()
                && !a1.is_nan()
                && !a2.is_nan()
                && !a0.is_nan()
                && !b0.is_nan()
                && !b1.is_nan()
                && !b2.is_nan()
        );

        self.a0 = a0;
        self.a1 = a1 / a0;
        self.a2 = a2 / a0;
        self.b0 = b0 / a0;
        self.b1 = b1 / a0;
        self.b2 = b2 / a0;

        assert!(
            !self.a0.is_nan()
                && !self.a1.is_nan()
                && !self.a2.is_nan()
                && !self.a0.is_nan()
                && !self.b0.is_nan()
                && !self.b1.is_nan()
                && !self.b2.is_nan()
        );
    }
}
