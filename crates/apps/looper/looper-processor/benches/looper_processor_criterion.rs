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
use std::time::Duration;

use audio_processor_testing_helpers::sine_buffer;
use criterion::measurement::WallTime;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkGroup, Criterion};

use audio_processor_traits::{AtomicF32, AudioProcessor, AudioProcessorSettings, VecAudioBuffer};
use looper_processor::parameters::{build_default_parameters, ParameterId};
use looper_processor::{parameters, LooperProcessor, MultiTrackLooper};

fn gain_vec(buffer: &mut Vec<f32>) {
    for sample in buffer {
        *sample *= 0.5;
    }
}

fn gain_atomicf32_vec(buffer: &mut Vec<AtomicF32>) {
    for sample in buffer {
        sample.set(sample.get() * 0.5);
    }
}

fn process_scenes_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Process scenes");

    group.bench_function("process_scenes", |b| {
        let mut looper = MultiTrackLooper::default();
        b.iter(|| {
            looper.process_scenes();
            black_box(&mut looper);
        })
    });
}

fn find_parameter_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Find parameter");

    group.bench_function("find_parameter_list", |b| {
        let (_hash_map, list) = build_default_parameters();
        let parameter_id: ParameterId =
            parameters::QuantizationParameter::QuantizationParameterTempoControl.into();
        b.iter(|| {
            let result = list.iter().find(|pid| **pid == parameter_id);
            black_box(result);
        })
    });

    group.bench_function("find_parameter_hashmap", |b| {
        let (hash_map, _list) = build_default_parameters();
        let parameter_id: ParameterId =
            parameters::QuantizationParameter::QuantizationParameterTempoControl.into();
        b.iter(|| {
            let result = hash_map.get(&parameter_id);
            black_box(result);
        })
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    print_time_limit(1000);
    print_time_limit(512);
    print_time_limit(128);
    print_time_limit(64);

    process_scenes_benchmark(c);
    find_parameter_benchmark(c);

    setup_multi_track_looper_bench(c);

    let mut group = c.benchmark_group("LooperProcessor");
    group.bench_function("Vec<f32> apply gain", |b| {
        let sine = sine_buffer(1000.0, 440.0, Duration::from_millis(1000));
        let mut buffer = sine;
        b.iter(|| {
            gain_vec(&mut buffer);
            black_box(&mut buffer);
        });
    });

    group.bench_function("Vec<AtomicF32> apply gain", |b| {
        let sine = sine_buffer(1000.0, 440.0, Duration::from_millis(1000))
            .into_iter()
            .map(AtomicF32::new)
            .collect();
        let mut buffer = sine;
        b.iter(|| {
            gain_atomicf32_vec(&mut buffer);
            black_box(&mut buffer);
        });
    });
    setup_processor_bench(&mut group, 1000);
    setup_processor_bench(&mut group, 512);
    setup_processor_bench(&mut group, 64);
}

fn setup_multi_track_looper_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("MultiTrackLooper");

    group.bench_function("MultiTrackLooper_512", |b| {
        let sine = sine_buffer(44100.0, 440.0, Duration::from_millis(1000));
        let mut buffer = VecAudioBuffer::new_with(sine, 1, 512);

        let mut processor = MultiTrackLooper::default();
        processor.prepare(AudioProcessorSettings::new(44100.0, 1, 1, 512));

        b.iter(|| {
            processor.process(&mut buffer);
            black_box(&mut buffer);
        });
    });

    group.bench_function("MultiTrackLooper_128", |b| {
        let sine = sine_buffer(44100.0, 440.0, Duration::from_millis(1000));
        let mut buffer = VecAudioBuffer::new_with(sine, 1, 128);

        let mut processor = MultiTrackLooper::default();
        processor.prepare(AudioProcessorSettings::new(44100.0, 1, 1, 128));

        b.iter(|| {
            processor.process(&mut buffer);
            black_box(&mut buffer);
        });
    });
}

fn setup_processor_bench(group: &mut BenchmarkGroup<WallTime>, buffer_size: usize) {
    let fbuffer_size = buffer_size as f32;

    group.bench_function(format!("LooperProcessor_process_{}", buffer_size), |b| {
        let sine = sine_buffer(fbuffer_size, 440.0, Duration::from_millis(1000));
        let mut buffer = VecAudioBuffer::new_with(sine, 1, buffer_size);

        let mut processor = LooperProcessor::default();
        processor.prepare(AudioProcessorSettings::new(fbuffer_size, 1, 1, buffer_size));
        processor.handle().set_tick_time(false);

        b.iter(|| {
            processor.process(&mut buffer);
            black_box(&mut buffer);
        });
    });

    group.bench_function(
        format!("LooperProcessor_process_{}_recording", buffer_size),
        |b| {
            let sine = sine_buffer(fbuffer_size, 440.0, Duration::from_millis(1000));
            let mut buffer = VecAudioBuffer::new_with(sine, 1, buffer_size);

            let mut processor = LooperProcessor::default();
            processor.prepare(AudioProcessorSettings::new(fbuffer_size, 1, 1, buffer_size));
            processor.handle().set_tick_time(false);
            processor.handle().start_recording();

            b.iter(|| {
                processor.process(&mut buffer);
                black_box(&mut buffer);
            });
        },
    );

    group.bench_function(
        format!("LooperProcessor_process_{}_playing", buffer_size),
        |b| {
            let sine = sine_buffer(fbuffer_size, 440.0, Duration::from_millis(1000));
            let mut buffer = VecAudioBuffer::new_with(sine, 1, buffer_size);

            let mut processor = LooperProcessor::default();
            processor.prepare(AudioProcessorSettings::new(fbuffer_size, 1, 1, buffer_size));
            processor.handle().set_tick_time(false);
            processor.handle().start_recording();
            processor.process(&mut buffer);
            processor.handle().stop_recording_allocating_loop();

            b.iter(|| {
                processor.process(&mut buffer);
                black_box(&mut buffer);
            });
        },
    );
}

fn print_time_limit(buffer_size: usize) {
    let fbuffer_size = buffer_size as f32;
    println!("================================================================================");
    println!("Processor Benchmarks\n* BufferSize={}", buffer_size);
    println!(
        "* Calculated time limit at 44.1kHz:\n    {}ms\n    {}us\n    {}ns",
        (fbuffer_size * (1.0 / 44100.0)) * 1000.0,
        (fbuffer_size * (1.0 / 44100.0)) * 1_000_000.0,
        (fbuffer_size * (1.0 / 44100.0)) * 1_000_000_000.0
    );
    println!("================================================================================");
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
