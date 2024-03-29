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
use audio_processor_traits::audio_buffer::{OwnedAudioBuffer, VecAudioBuffer};
use audio_processor_traits::{AudioBuffer, Float};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const NUM_SAMPLES: usize = 512;

fn gain_vec(vec: &mut Vec<f32>) {
    for sample in vec {
        *sample = 0.1 * *sample * *sample * *sample * *sample * *sample;
    }
}

fn gain_buffer_slice_mut<BufferType>(audio_buffer: &mut BufferType)
where
    BufferType: AudioBuffer<SampleType = f32>,
{
    for sample in audio_buffer.slice_mut() {
        *sample = 0.1 * *sample * *sample * *sample * *sample * *sample;
    }
}

fn unsafe_gain_buffer_fixed_single_channel<BufferType>(audio_buffer: &mut BufferType)
where
    BufferType: AudioBuffer<SampleType = f32>,
{
    unsafe {
        for sample_index in 0..audio_buffer.num_samples() {
            let sample_ref = audio_buffer.get_unchecked_mut(0, sample_index);
            let sample = *sample_ref;
            let output = sample * 0.1 * sample * sample * sample * sample;
            *sample_ref = output;
        }
    }
}

fn gain_buffer_fixed_single_channel<BufferType>(audio_buffer: &mut BufferType)
where
    BufferType: AudioBuffer<SampleType = f32>,
{
    for sample_index in 0..audio_buffer.num_samples() {
        let sample = *audio_buffer.get(0, sample_index);
        let output = sample * 0.1 * sample * sample * sample * sample;
        audio_buffer.set(0, sample_index, output);
    }
}

fn gain_buffer_fixed_single_channel_ref<BufferType>(audio_buffer: &mut BufferType)
where
    BufferType: AudioBuffer<SampleType = f32>,
{
    for sample_index in 0..audio_buffer.num_samples() {
        let sample = audio_buffer.get_mut(0, sample_index);
        *sample = *sample * 0.1 * *sample * *sample * *sample * *sample;
    }
}

fn gain_buffer_fixed<BufferType>(audio_buffer: &mut BufferType)
where
    BufferType: AudioBuffer<SampleType = f32>,
{
    for sample_index in 0..audio_buffer.num_samples() {
        for channel_index in 0..audio_buffer.num_channels() {
            let sample = *audio_buffer.get(channel_index, sample_index);
            let output = sample * 0.1 * sample * sample * sample * sample;
            audio_buffer.set(channel_index, sample_index, output);
        }
    }
}

fn gain_buffer<SampleType, BufferType>(audio_buffer: &mut BufferType)
where
    BufferType: AudioBuffer<SampleType = SampleType>,
    SampleType: Float,
{
    for sample_index in 0..audio_buffer.num_samples() {
        for channel_index in 0..audio_buffer.num_channels() {
            let sample = *audio_buffer.get(channel_index, sample_index);
            let output =
                sample * SampleType::from(0.1).unwrap() * sample * sample * sample * sample;
            audio_buffer.set(channel_index, sample_index, output);
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("AudioBuffer - 512 samples (11ms)");

    group.bench_function("VEC - apply gain - 512 samples 11ms to process", |b| {
        let sine = sine_wave_vec(NUM_SAMPLES);
        let mut buffer = sine;
        b.iter(|| {
            gain_vec(&mut buffer);
            black_box(&mut buffer);
        });
    });

    group.bench_function(
        "VecAudioBuffer::slice_mut - apply gain - 512 samples 11ms to process",
        |b| {
            let mut buffer = VecAudioBuffer::new();
            buffer.resize(1, NUM_SAMPLES, 0.0_f32);
            let sine = sine_wave_vec(NUM_SAMPLES);
            for sample_index in 0..buffer.num_samples() {
                buffer.set(0, sample_index, sine[sample_index]);
            }
            b.iter(|| {
                gain_buffer_slice_mut(&mut buffer);
                black_box(&mut buffer);
            });
        },
    );

    group.bench_function(
        "VecAudioBuffer - apply gain with fixed sample type - 512 samples 11ms to process",
        |b| {
            let mut buffer = VecAudioBuffer::new();
            buffer.resize(1, NUM_SAMPLES, 0.0_f32);
            let sine = sine_wave_vec(NUM_SAMPLES);
            for sample_index in 0..buffer.num_samples() {
                buffer.set(0, sample_index, sine[sample_index]);
            }
            b.iter(|| {
                gain_buffer_fixed(&mut buffer);
                black_box(&mut buffer);
            });
        },
    );

    group.bench_function(
        "VecAudioBuffer - unsafe, fixed types, single channel - 512 samples 11ms to process",
        |b| {
            let mut buffer = build_sine_audio_buffer();
            b.iter(|| {
                unsafe_gain_buffer_fixed_single_channel(&mut buffer);
                black_box(&mut buffer);
            });
        },
    );

    group.bench_function(
        "VecAudioBuffer - apply gain with fixed sample type & single channel - 512 samples 11ms to process",
        |b| {
            let mut buffer = build_sine_audio_buffer();
            b.iter(|| {
                gain_buffer_fixed_single_channel(&mut buffer);
                black_box(&mut buffer);
            });
        },
    );

    group.bench_function(
        "VecAudioBuffer - apply gain with any sample type - 512 samples 11ms to process",
        |b| {
            let mut buffer = build_sine_audio_buffer();
            b.iter(|| {
                gain_buffer(&mut buffer);
                black_box(&mut buffer);
            });
        },
    );

    group.bench_function(
        "VecAudioBuffer - apply gain with mutable ref - 512 samples 11ms to process",
        |b| {
            let mut buffer = build_sine_audio_buffer();
            b.iter(|| {
                gain_buffer_fixed_single_channel_ref(&mut buffer);
                black_box(&mut buffer);
            });
        },
    );

    group.bench_function("Copy 512 samples to buffer", |b| {
        let mut buffer_l = sine_wave_vec(512);
        let mut buffer_r = sine_wave_vec(512);
        let mut vec_buffer = VecAudioBuffer::new_with_size(2, 2000, 0.0);

        b.iter(|| {
            for sample_index in 0..buffer_l.len() {
                vec_buffer.set(0, sample_index, buffer_l[sample_index]);
                vec_buffer.set(1, sample_index, buffer_r[sample_index]);
            }

            black_box(&mut vec_buffer);
            black_box(&mut buffer_l);
            black_box(&mut buffer_r);
        });
    });
}

fn build_sine_audio_buffer() -> VecAudioBuffer<f32> {
    let mut buffer = VecAudioBuffer::new();
    buffer.resize(1, NUM_SAMPLES, 0.0_f32);
    let sine = sine_wave_vec(NUM_SAMPLES);
    for sample_index in 0..buffer.num_samples() {
        buffer.set(0, sample_index, sine[sample_index]);
    }
    buffer
}

fn sine_wave_vec(duration_samples: usize) -> Vec<f32> {
    let mut oscillator = augmented_oscillator::Oscillator::sine(44100.0);
    oscillator.set_frequency(440.0);
    let mut output_buffer = Vec::new();
    output_buffer.resize(duration_samples, 0.0);
    for sample in &mut output_buffer {
        *sample = oscillator.get();
        oscillator.tick();
    }
    output_buffer
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
