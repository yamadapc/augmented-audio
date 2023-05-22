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

use audio_processor_traits::num::pow::Pow;
use audio_processor_traits::Float;
use std::f64::consts::PI;

pub type WindowFunction<F> = fn(n: F, size: F) -> F;

pub enum WindowFunctionType {
    Hann,
    BlackmanHarris,
    Blackman,
    Rectangular,
    Triangular,
    Parzen,
    Welch,
    Sine,
}

pub fn make_window_vec<F: Float>(size: usize, fn_type: WindowFunctionType) -> Vec<F> {
    match fn_type {
        WindowFunctionType::Hann => make_hann_vec(size),
        WindowFunctionType::BlackmanHarris => make_blackman_harris_vec(size),
        WindowFunctionType::Blackman => make_blackman_vec(size),
        WindowFunctionType::Rectangular => make_rectangular_vec(size),
        WindowFunctionType::Triangular => make_triangular_vec(size),
        WindowFunctionType::Parzen => make_parzen_vec(size),
        WindowFunctionType::Welch => make_welch_vec(size),
        WindowFunctionType::Sine => make_sine_vec(size),
    }
}

pub fn hann(n: f64, size: f64) -> f64 {
    0.5 * (1.0 - (2.0 * PI * (n / size)).cos())
}

pub fn blackman_harris(n: f64, size: f64) -> f64 {
    let a0 = 0.35875;
    let a1 = 0.48829;
    let a2 = 0.14128;
    let a3 = 0.01168;
    a0 - a1 * ((2.0 * PI * n) / size).cos() + a2 * ((4.0 * PI * n) / size).cos()
        - a3 * ((6.0 * PI * n) / size).cos()
}

pub fn blackman(n: f64, size: f64) -> f64 {
    // let alpha = 0.16;
    let a0 = 0.42; // (1.0 - alpha) / 2.0;
    let a1 = 0.5; // 1.0 / 2.0;
    let a2 = 0.08; // alpha / 2.0;
    a0 - a1 * ((2.0 * PI * n) / size).cos() + a2 * ((4.0 * PI * n) / size).cos()
}

pub fn triangular(n: f64, size: f64) -> f64 {
    let l_value = size;
    1.0 - ((n - size / 2.0) / (l_value)).abs()
}

pub fn parzen(n: f64, size: f64) -> f64 {
    let l = size + 1.0;
    let w0 = |n: f64| {
        let mn = n.abs();
        if mn <= l / 4.0 {
            1.0f64 - 6.0f64 * (n / (l / 2.0)).pow(2) * (1.0 - mn / (l / 2.0))
        } else if mn <= l / 2.0 {
            2.0f64 * (1.0 - mn / (l / 2.0)).pow(3)
        } else {
            1.0
        }
    };
    w0(n - size / 2.0)
}

pub fn welch(n: f64, size: f64) -> f64 {
    1.0f64 - ((n - size / 2.0) / (size / 2.0)).pow(2)
}

pub fn sine(n: f64, size: f64) -> f64 {
    (PI * (n / size)).sin()
}

fn make_vec<F: Float>(size: usize, window_fn: WindowFunction<f64>) -> Vec<F> {
    (0..size)
        .map(|n| window_fn(n as f64, size as f64))
        .map(|n| F::from(n).unwrap())
        .collect()
}

pub fn make_hann_vec<F: Float>(size: usize) -> Vec<F> {
    make_vec(size, hann)
}

pub fn make_blackman_harris_vec<F: Float>(size: usize) -> Vec<F> {
    make_vec(size, blackman_harris)
}

pub fn make_blackman_vec<F: Float>(size: usize) -> Vec<F> {
    make_vec(size, blackman)
}

pub fn make_rectangular_vec<F: Float>(size: usize) -> Vec<F> {
    (0..size).map(|_| F::from(1.0).unwrap()).collect()
}

pub fn make_triangular_vec<F: Float>(size: usize) -> Vec<F> {
    make_vec(size, triangular)
}

pub fn make_parzen_vec<F: Float>(size: usize) -> Vec<F> {
    make_vec(size, parzen)
}

pub fn make_welch_vec<F: Float>(size: usize) -> Vec<F> {
    make_vec(size, welch)
}

pub fn make_sine_vec<F: Float>(size: usize) -> Vec<F> {
    make_vec(size, sine)
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::{charts::draw_vec_chart, relative_path};

    use super::*;

    type WindowFunction = fn(usize) -> Vec<f32>;
    fn window_functions() -> Vec<(&'static str, WindowFunction)> {
        vec![
            ("RectangularWindow", make_rectangular_vec),
            ("TriangularWindow", make_triangular_vec),
            ("HannWindow", make_hann_vec),
            ("ParzenWindow", make_parzen_vec),
            ("WelchWindow", make_welch_vec),
            ("SineWindow", make_sine_vec),
            ("BlackmanWindow", make_blackman_vec),
            ("BlackmanHarrisWindow", make_blackman_harris_vec),
        ]
    }

    fn draw_window(label: &str, window_fn: WindowFunction) {
        let window = window_fn(1000);
        draw_vec_chart(
            &relative_path!("src/window_functions/windows"),
            label,
            window,
        );
    }

    #[test]
    fn test_draw_windows() {
        for (label, window_fn) in window_functions() {
            draw_window(label, window_fn);
        }
    }
}
