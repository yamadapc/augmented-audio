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
    let mut oscillator = oscillator::Oscillator::sine(44100.0);
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

    let mut oscillator = oscillator::Oscillator::sine(44100.0);
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
