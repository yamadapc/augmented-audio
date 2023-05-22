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

use audio_processor_traits::num::traits::FloatConst;
use audio_processor_traits::Float;

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

pub fn make_window_vec<F: Float + FloatConst>(size: usize, fn_type: WindowFunctionType) -> Vec<F> {
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

#[numeric_literals::replace_float_literals(F::from(literal).unwrap())]
pub fn hann<F>(n: F, size: F) -> F
where
    F: Float + FloatConst,
{
    0.5 * (1.0 - (2.0 * F::PI() * (n / size)).cos())
}

#[numeric_literals::replace_float_literals(F::from(literal).unwrap())]
pub fn blackman_harris<F>(n: F, size: F) -> F
where
    F: Float + FloatConst,
{
    let pi = F::PI();
    let a0 = 0.35875;
    let a1 = 0.48829;
    let a2 = 0.14128;
    let a3 = 0.01168;
    a0 - a1 * ((2.0 * pi * n) / size).cos() + a2 * ((4.0 * pi * n) / size).cos()
        - a3 * ((6.0 * pi * n) / size).cos()
}

#[numeric_literals::replace_float_literals(F::from(literal).unwrap())]
pub fn blackman<F>(n: F, size: F) -> F
where
    F: Float + FloatConst,
{
    // let alpha = 0.16;
    let pi = F::PI();
    let a0 = 0.42; // (1.0 - alpha) / 2.0;
    let a1 = 0.5; // 1.0 / 2.0;
    let a2 = 0.08; // alpha / 2.0;
    a0 - a1 * ((2.0 * pi * n) / size).cos() + a2 * ((4.0 * pi * n) / size).cos()
}

#[numeric_literals::replace_float_literals(F::from(literal).unwrap())]
pub fn triangular<F>(n: F, size: F) -> F
where
    F: Float,
{
    let l_value = size;
    1.0 - ((n - size / 2.0) / (l_value)).abs()
}

#[numeric_literals::replace_float_literals(F::from(literal).unwrap())]
pub fn parzen<F>(n: F, size: F) -> F
where
    F: Float,
{
    let l = size + 1.0;
    let w0 = |n: F| {
        let mn = n.abs();
        if mn <= l / 4.0 {
            1.0f64 - 6.0f64 * (n / (l / 2.0)).powi(2) * (1.0 - mn / (l / 2.0))
        } else if mn <= l / 2.0 {
            2.0f64 * (1.0 - mn / (l / 2.0)).powi(3)
        } else {
            1.0
        }
    };
    w0(n - size / 2.0)
}

#[numeric_literals::replace_float_literals(F::from(literal).unwrap())]
pub fn welch<F>(n: F, size: F) -> F
where
    F: Float,
{
    1.0f64 - ((n - size / 2.0) / (size / 2.0)).powi(2)
}

pub fn sine<F>(n: F, size: F) -> F
where
    F: Float + FloatConst,
{
    (F::PI() * (n / size)).sin()
}

fn make_vec<F>(size: usize, window_fn: WindowFunction<F>) -> Vec<F>
where
    F: Float,
{
    (0..size)
        .map(|n| window_fn(F::from(n).unwrap(), F::from(size).unwrap()))
        .collect()
}

pub fn make_hann_vec<F: Float + FloatConst>(size: usize) -> Vec<F> {
    make_vec(size, hann)
}

pub fn make_blackman_harris_vec<F: Float + FloatConst>(size: usize) -> Vec<F> {
    make_vec(size, blackman_harris)
}

pub fn make_blackman_vec<F: Float + FloatConst>(size: usize) -> Vec<F> {
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

pub fn make_sine_vec<F: Float + FloatConst>(size: usize) -> Vec<F> {
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
