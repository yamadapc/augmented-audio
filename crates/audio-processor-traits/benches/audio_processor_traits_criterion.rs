use audio_processor_traits::audio_buffer::{OwnedAudioBuffer, SliceAudioBuffer, VecAudioBuffer};
use audio_processor_traits::{AudioBuffer, Float};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const NUM_SAMPLES: usize = 512;

fn gain_vec(vec: &mut Vec<f32>) {
    for sample in vec {
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
    c.bench_function("VEC - apply gain - 512 samples 11ms to process", |b| {
        let mut buffer = sine_wave_vec(NUM_SAMPLES);
        b.iter(|| {
            gain_vec(&mut buffer);
            black_box(&mut buffer);
        });
    });

    c.bench_function(
        "VecAudioBuffer - apply gain with fixed sample type - 512 samples 11ms to process",
        |b| {
            let mut buffer = VecAudioBuffer::new();
            buffer.resize(1, NUM_SAMPLES, 0.0 as f32);
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

    c.bench_function(
        "VecAudioBuffer - unsafe, fixed types, single channel - 512 samples 11ms to process",
        |b| {
            let mut buffer = VecAudioBuffer::new();
            buffer.resize(1, NUM_SAMPLES, 0.0 as f32);
            let sine = sine_wave_vec(NUM_SAMPLES);
            for sample_index in 0..buffer.num_samples() {
                buffer.set(0, sample_index, sine[sample_index]);
            }
            b.iter(|| {
                unsafe_gain_buffer_fixed_single_channel(&mut buffer);
                black_box(&mut buffer);
            });
        },
    );

    c.bench_function(
        "VecAudioBuffer - apply gain with fixed sample type & single channel - 512 samples 11ms to process",
        |b| {
            let mut buffer = VecAudioBuffer::new();
            buffer.resize(1, NUM_SAMPLES, 0.0 as f32);
            let sine = sine_wave_vec(NUM_SAMPLES);
            for sample_index in 0..buffer.num_samples() {
                buffer.set(0, sample_index, sine[sample_index]);
            }
            b.iter(|| {
                gain_buffer_fixed_single_channel(&mut buffer);
                black_box(&mut buffer);
            });
        },
    );

    c.bench_function(
        "VecAudioBuffer - apply gain with any sample type - 512 samples 11ms to process",
        |b| {
            let mut buffer = VecAudioBuffer::new();
            buffer.resize(1, NUM_SAMPLES, 0.0 as f32);
            let sine = sine_wave_vec(NUM_SAMPLES);
            for sample_index in 0..buffer.num_samples() {
                buffer.set(0, sample_index, sine[sample_index]);
            }
            b.iter(|| {
                gain_buffer(&mut buffer);
                black_box(&mut buffer);
            });
        },
    );

    c.bench_function(
        "VecAudioBuffer - apply gain with mutable ref - 512 samples 11ms to process",
        |b| {
            let mut buffer = VecAudioBuffer::new();
            buffer.resize(1, NUM_SAMPLES, 0.0 as f32);
            let sine = sine_wave_vec(NUM_SAMPLES);
            for sample_index in 0..buffer.num_samples() {
                buffer.set(0, sample_index, sine[sample_index]);
            }
            b.iter(|| {
                gain_buffer_fixed_single_channel_ref(&mut buffer);
                black_box(&mut buffer);
            });
        },
    );

    c.bench_function(
        "SliceAudioBuffer - apply gain with fixed sample type & single channel - 512 samples 11ms to process",
        |b| {
            let mut sine = sine_wave_vec(NUM_SAMPLES);
            let mut channels = [sine.as_mut_slice()];
            let mut buffer = SliceAudioBuffer::new(&mut channels);
            b.iter(|| {
                gain_buffer_fixed_single_channel(&mut buffer);
                black_box(&mut buffer);
            });
        },
    );

    c.bench_function(
        "SliceAudioBuffer - apply gain with any sample type - 512 samples 11ms to process",
        |b| {
            let mut sine = sine_wave_vec(NUM_SAMPLES);
            let mut channels = [sine.as_mut_slice()];
            let mut buffer = SliceAudioBuffer::new(&mut channels);
            b.iter(|| {
                gain_buffer(&mut buffer);
                black_box(&mut buffer);
            });
        },
    );
}

fn sine_wave_vec(duration_samples: usize) -> Vec<f32> {
    let mut oscillator = oscillator::Oscillator::sine(44100.0);
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
