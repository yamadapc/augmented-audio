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
use augmented_oscillator::generators::{saw_generator, square_generator};
use augmented_oscillator::wavetable::WaveTableOscillator;
use augmented_oscillator::Oscillator;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn sine_osc(oscillator: &mut Oscillator<f32>, output_buffer: &mut [f32]) {
    for i in output_buffer {
        *i = oscillator.next_sample();
    }
}

fn wave_table_sine_osc(wave_table_oscillator: &mut WaveTableOscillator, output_buffer: &mut [f32]) {
    for i in output_buffer {
        *i = wave_table_oscillator.next_sample();
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sine_wave_table_osc - 10k samples", |b| {
        let mut oscillator = Oscillator::sine(44100.0);
        oscillator.set_frequency(440.0);
        let mut wave_table_oscillator = WaveTableOscillator::from_oscillator(oscillator, 1000);
        let mut output_buffer = Vec::new();
        output_buffer.resize(10000, 0.0);

        b.iter(|| wave_table_sine_osc(&mut wave_table_oscillator, black_box(&mut output_buffer)))
    });

    c.bench_function("sine_osc - 10k samples", |b| {
        let mut oscillator = Oscillator::sine(44100.0);
        oscillator.set_frequency(440.0);
        let mut output_buffer = Vec::new();
        output_buffer.resize(10000, 0.0);

        b.iter(|| sine_osc(&mut oscillator, black_box(&mut output_buffer)))
    });

    c.bench_function("saw_osc - 10k samples", |b| {
        let mut oscillator = Oscillator::new_with_sample_rate(44100.0, saw_generator);
        oscillator.set_frequency(440.0);
        let mut output_buffer = Vec::new();
        output_buffer.resize(10000, 0.0);

        b.iter(|| sine_osc(&mut oscillator, black_box(&mut output_buffer)))
    });

    c.bench_function("square_osc - 10k samples", |b| {
        let mut oscillator = Oscillator::new_with_sample_rate(44100.0, square_generator);
        oscillator.set_frequency(440.0);
        let mut output_buffer = Vec::new();
        output_buffer.resize(10000, 0.0);

        b.iter(|| sine_osc(&mut oscillator, black_box(&mut output_buffer)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
