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
use criterion::{black_box, Criterion};

fn rms_abs(buffer: &Vec<f32>) -> f32 {
    let mut sum = 0.0;
    for sample in buffer {
        sum += sample.abs();
    }
    sum / buffer.len() as f32
}

fn rms_pow(buffer: &Vec<f32>) -> f32 {
    let mut sum = 0.0;
    for sample in buffer {
        sum += sample * sample;
    }
    sum.sqrt() / buffer.len() as f32
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("TestHostPlugin - RMS");
    let mut oscillator = augmented_oscillator::Oscillator::sine(44100.0);
    oscillator.set_frequency(440.0);
    let mut output_buffer = Vec::new();
    output_buffer.resize(400000, 0.0);
    for sample in &mut output_buffer {
        *sample = oscillator.get();
        oscillator.tick();
    }

    group.bench_function("`abs` - stress 10s of audio at 44.1kHz", |b| {
        b.iter(|| rms_abs(black_box(&mut output_buffer)))
    });

    group.bench_function("`sq root - stress 10s of audio` at 44.1kHz", |b| {
        b.iter(|| rms_pow(black_box(&mut output_buffer)))
    });

    let mut oscillator = augmented_oscillator::Oscillator::sine(44100.0);
    oscillator.set_frequency(440.0);
    let mut output_buffer = Vec::new();
    output_buffer.resize(512, 0.0);
    for sample in &mut output_buffer {
        *sample = oscillator.get();
        oscillator.tick();
    }

    group.bench_function("`abs` - 512 samples 11ms to process at 44.1kHz", |b| {
        b.iter(|| rms_abs(black_box(&mut output_buffer)))
    });

    group.bench_function("`sq root` - 512 samples 11ms to process at 44.1kHz", |b| {
        b.iter(|| rms_pow(black_box(&mut output_buffer)))
    });
}
