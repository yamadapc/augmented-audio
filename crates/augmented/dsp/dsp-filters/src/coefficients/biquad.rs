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

use num::{Complex, Float};
use std::f64::consts::PI;

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

fn add_mul<T: Float>(c: Complex<T>, v: T, c1: Complex<T>) -> Complex<T> {
    Complex::new(c.re + v * c1.re, c.im + v * c1.im)
}

impl<Sample: Float> BiquadCoefficients<Sample> {
    /// Calculate the filter response to a given frequency. Useful for drawing frequency response
    /// charts.
    ///
    /// Takes in a normalized_frequency between 0 and 1.
    pub fn response(&self, normalized_frequency: Sample) -> Complex<f64> {
        let w: f64 = 2.0 * PI * normalized_frequency.to_f64().unwrap();
        let czn1 = Complex::from_polar(1., -w);
        let czn2 = Complex::from_polar(1., -2.0 * w);
        let mut ch = Complex::new(1.0, 0.0);
        let mut cbot = Complex::new(1.0, 0.0);

        let a0: f64 = self.a0.to_f64().unwrap();
        let b0: f64 = self.b0.to_f64().unwrap();
        let b1: f64 = self.b1.to_f64().unwrap();
        let b2: f64 = self.b2.to_f64().unwrap();
        let a1: f64 = self.a1.to_f64().unwrap();
        let a2: f64 = self.a2.to_f64().unwrap();

        let mut ct = Complex::new(b0 / a0, 0.0);
        let mut cb = Complex::new(1.0, 0.0);
        ct = add_mul(ct, b1 / a0, czn1);
        ct = add_mul(ct, b2 / a0, czn2);
        cb = add_mul(cb, a1 / a0, czn1);
        cb = add_mul(cb, a2 / a0, czn2);
        ch *= ct;
        cbot *= cb;

        ch / cbot
    }

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
