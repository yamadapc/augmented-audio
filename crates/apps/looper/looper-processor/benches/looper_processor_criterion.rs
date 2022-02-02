use audio_processor_testing_helpers::sine_buffer;
use audio_processor_traits::{AtomicF32, AudioProcessor, AudioProcessorSettings, VecAudioBuffer};
use criterion::measurement::WallTime;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkGroup, Criterion};
use looper_processor::LooperProcessor;
use std::time::Duration;

fn gain_vec(buffer: &mut Vec<f32>) {
    for sample in buffer {
        *sample = 0.5 * *sample;
    }
}

fn gain_atomicf32_vec(buffer: &mut Vec<AtomicF32>) {
    for sample in buffer {
        sample.set(sample.get() * 0.5);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
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
            .map(|i| AtomicF32::new(i))
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
        (fbuffer_size * (1.0 / 44100.0)) * 1000_000.0,
        (fbuffer_size * (1.0 / 44100.0)) * 1000_000_000.0
    );
    println!("================================================================================");

    group.bench_function(
        format!("LooperProcessor::process ({} samples chunk)", buffer_size),
        |b| {
            let sine = sine_buffer(fbuffer_size, 440.0, Duration::from_millis(1000));
            let mut buffer = VecAudioBuffer::new_with(sine, 1, buffer_size);

            let mut processor = LooperProcessor::new(audio_garbage_collector::handle(), None);
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

            let mut processor = LooperProcessor::new(audio_garbage_collector::handle(), None);
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

            let mut processor = LooperProcessor::new(audio_garbage_collector::handle(), None);
            processor.prepare(AudioProcessorSettings::new(fbuffer_size, 1, 1, buffer_size));
            processor.handle().start_recording();
            processor.process(&mut buffer);
            processor.handle().stop_recording();

            b.iter(|| {
                processor.process(&mut buffer);
                black_box(&mut buffer);
            });
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
