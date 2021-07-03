use std::time::Duration;

use oscillator::generators::sine_generator;
use oscillator::Oscillator;

/// Create a sine wave buffer with this duration
pub fn sine_buffer(sample_rate: f32, frequency: f32, length: Duration) -> Vec<f32> {
    let mut source = Oscillator::new(sine_generator);
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

/// Get RMS level for a buffer
pub fn rms_level(input: &[f32]) -> f32 {
    if input.is_empty() {
        return 0.0;
    }
    let mut s = 0.0;
    for i in input {
        s += i.abs();
    }
    s / (input.len() as f32)
}

/// Test two buffers have equivalent RMS levels
pub fn test_level_equivalence(
    input_buffer: &Vec<f32>,
    output_buffer: &[f32],
    input_window_size: usize,
    output_window_size: usize,
    threshold: f32,
) {
    let input_chunks = input_buffer.chunks(input_window_size);
    let output_chunks = output_buffer.chunks(output_window_size);
    assert!(input_buffer.len() > 0);
    assert!(output_buffer.len() > 0);
    // assert!((input_chunks.len() as i32 - output_chunks.len() as i32).abs() < 2);
    for (input_chunk, output_chunk) in input_chunks.zip(output_chunks) {
        let input_level = rms_level(input_chunk);
        let output_level = rms_level(output_chunk);
        let diff = (input_level - output_level).abs();
        println!(
            "  -> Input level: {} Output level: {} Diff: {}",
            input_level, output_level, diff
        );
        assert!(diff < threshold);
    }
}
