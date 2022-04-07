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
pub use augmented_oscillator::generators::*;
pub use augmented_oscillator::Oscillator;
use std::time::Duration;

/// Create a sine wave buffer with this duration
pub fn sine_buffer(sample_rate: f32, frequency: f32, length: Duration) -> Vec<f32> {
    oscillator_buffer(sample_rate, frequency, length, sine_generator)
}

/// Create a sine wave buffer with this duration
pub fn oscillator_buffer(
    sample_rate: f32,
    frequency: f32,
    length: Duration,
    generator_fn: fn(f32) -> f32,
) -> Vec<f32> {
    let mut source = Oscillator::new(generator_fn);
    source.set_sample_rate(sample_rate);
    source.set_frequency(frequency);
    let mut output = Vec::new();
    let length_samples = (length.as_secs_f32() * sample_rate).ceil();
    output.resize(length_samples as usize, 0.0);
    for sample in &mut output {
        *sample = source.next_sample();
    }
    output
}
