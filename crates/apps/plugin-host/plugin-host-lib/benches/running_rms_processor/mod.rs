use audio_processor_traits::audio_buffer::{OwnedAudioBuffer, VecAudioBuffer};
use audio_processor_traits::{
    simple_processor::process_buffer, AtomicF32, AudioBuffer,
    AudioProcessorSettings, SimpleAudioProcessor,
};
use criterion::{black_box, Criterion};
use plugin_host_lib::processors::running_rms_processor::RunningRMSProcessor;
use std::time::Duration;

fn process_with_atomic<Buffer: AudioBuffer<SampleType = f32>>(
    audio_buffer: &mut Buffer,
    output_buffer: &mut Vec<AtomicF32>,
) {
    for (sample_index, frame) in audio_buffer.frames().enumerate() {
        for sample in frame {
            let value_slot = &output_buffer[sample_index];
            let new_value = *sample * *sample;
            value_slot.set(new_value);
        }
    }
}

fn process_with_f32<Buffer: AudioBuffer<SampleType = f32>>(
    audio_buffer: &mut Buffer,
    output_buffer: &mut Vec<f32>,
) {
    for (sample_index, frame) in audio_buffer.frames().enumerate() {
        for sample in frame {
            output_buffer[sample_index] = *sample * *sample;
        }
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("TestHostPlugin - RunningRMSProcessor");
    let mut oscillator = augmented_oscillator::Oscillator::sine(44100.0);
    oscillator.set_frequency(440.0);
    let mut audio_buffer = VecAudioBuffer::new();
    audio_buffer.resize(2, 512, 0.0);
    for frame in audio_buffer.frames_mut() {
        frame[0] = oscillator.get();
        frame[1] = oscillator.get();
        oscillator.tick();
    }
    let garbage_collector = audio_garbage_collector::GarbageCollector::default();
    let mut processor = RunningRMSProcessor::new_with_duration(
        garbage_collector.handle(),
        Duration::from_millis(300),
    );
    processor.s_prepare(AudioProcessorSettings {
        sample_rate: 44100.,
        input_channels: 2,
        output_channels: 2,
        block_size: 512,
    });

    group.bench_function("process", |b| {
        b.iter(|| {
            process_buffer(&mut processor, &mut audio_buffer);
            black_box(&mut audio_buffer);
        })
    });

    group.bench_function("process with atomic", |b| {
        let mut atomic_buffer = Vec::new();
        atomic_buffer.resize(512, AtomicF32::new(0.));
        b.iter(|| {
            process_with_atomic(&mut audio_buffer, &mut atomic_buffer);
            black_box(&mut audio_buffer);
            black_box(&mut atomic_buffer);
        })
    });
    group.bench_function("process with f32", |b| {
        let mut f32_buffer = Vec::new();
        f32_buffer.resize(512, 0.);
        b.iter(|| {
            process_with_f32(&mut audio_buffer, &mut f32_buffer);
            black_box(&mut audio_buffer);
            black_box(&mut f32_buffer);
        })
    });
}
