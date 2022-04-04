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
    process_scenes_benchmark(c);
    find_parameter_benchmark(c);

    let mut group = c.benchmark_group("AudioBuffer - 512 samples (11ms)");

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

fn setup_processor_bench(group: &mut BenchmarkGroup<WallTime>, buffer_size: usize) {
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

    group.bench_function(
        format!("LooperProcessor::process ({} samples chunk)", buffer_size),
        |b| {
            let sine = sine_buffer(fbuffer_size, 440.0, Duration::from_millis(1000));
            let mut buffer = VecAudioBuffer::new_with(sine, 1, buffer_size);

            let mut processor = LooperProcessor::default();
            processor.prepare(AudioProcessorSettings::new(fbuffer_size, 1, 1, buffer_size));

            b.iter(|| {
                processor.process(&mut buffer);
                black_box(&mut buffer);
            });
        },
    );

    group.bench_function(
        format!(
            "LooperProcessor::process recording ({} samples chunk)",
            buffer_size
        ),
        |b| {
            let sine = sine_buffer(fbuffer_size, 440.0, Duration::from_millis(1000));
            let mut buffer = VecAudioBuffer::new_with(sine, 1, buffer_size);

            let mut processor = LooperProcessor::default();
            processor.prepare(AudioProcessorSettings::new(fbuffer_size, 1, 1, buffer_size));
            processor.handle().start_recording();

            b.iter(|| {
                processor.process(&mut buffer);
                black_box(&mut buffer);
            });
        },
    );

    group.bench_function(
        format!(
            "LooperProcessor::process playback ({} samples chunk)",
            buffer_size
        ),
        |b| {
            let sine = sine_buffer(fbuffer_size, 440.0, Duration::from_millis(1000));
            let mut buffer = VecAudioBuffer::new_with(sine, 1, buffer_size);

            let mut processor = LooperProcessor::default();
            processor.prepare(AudioProcessorSettings::new(fbuffer_size, 1, 1, buffer_size));
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

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
