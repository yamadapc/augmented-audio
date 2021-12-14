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
