pub use augmented_oscillator::generators::*;
pub use augmented_oscillator::Oscillator;
use std::time::Duration;

/// Create a sine wave buffer with this duration
pub fn sine_buffer(sample_rate: f32, frequency: f32, length: Duration) -> Vec<f32> {
    oscillator_buffer(sample_rate, frequency, length, sine_generator)
}

/// Create a sine wave buffer with this duration
pub fn oscillator_buffer(
    sample_rate: f32,
    frequency: f32,
    length: Duration,
    generator_fn: fn(f32) -> f32,
) -> Vec<f32> {
    let mut source = Oscillator::new(generator_fn);
    source.set_sample_rate(sample_rate);
    source.set_frequency(frequency);
    let mut output = Vec::new();
    let length_samples = (length.as_secs_f32() * sample_rate).ceil();
    output.resize(length_samples as usize, 0.0);
    for sample in &mut output {
        *sample = source.next_sample();
    }
    output
}
