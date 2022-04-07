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
use audio_processor_traits::{AudioBuffer, AudioProcessorSettings};
use criterion::{black_box, Criterion};
use plugin_host_lib::audio_io::cpal_vst_buffer_handler::CpalVstBufferHandler;
use plugin_host_lib::processors::test_host_processor::flush_vst_output;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("CpalVstBufferHandler");
    let mut oscillator = augmented_oscillator::Oscillator::sine(44100.0);
    oscillator.set_frequency(440.0);
    let mut output_buffer = VecAudioBuffer::new();
    output_buffer.resize(2, 512, 0.0);
    for sample_index in 0..output_buffer.num_samples() {
        let sample = oscillator.get();
        oscillator.tick();
        for channel_index in 0..output_buffer.num_channels() {
            output_buffer.set(channel_index, sample_index, sample)
        }
    }
    let settings = AudioProcessorSettings::new(1000.0, 2, 2, 512);
    let mut buffer_handler = CpalVstBufferHandler::new(settings);

    group.bench_function(
        "cpal buffer conversion - 512 samples 11ms to process at 44.1kHz",
        |b| {
            b.iter(|| {
                buffer_handler.process(black_box(&mut output_buffer));
                black_box(buffer_handler.get_audio_buffer());
            })
        },
    );

    buffer_handler.process(&mut output_buffer);
    let mut audio_buffer = buffer_handler.get_audio_buffer();
    let mut output = VecAudioBuffer::new();
    output.resize(audio_buffer.input_count(), audio_buffer.samples(), 0.0);

    group.bench_function(
        "flush_vst_output - 512 samples 11ms to process at 44.1kHz",
        |b| {
            b.iter(|| {
                flush_vst_output(2, &mut audio_buffer, &mut output);
                black_box(&mut output);
                black_box(&mut audio_buffer);
            })
        },
    );
}
