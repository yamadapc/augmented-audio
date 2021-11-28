pub use generators::sine_buffer;
pub use util::rms_level;

pub mod charts;
mod generators;
mod util;

/// Test two buffers have equivalent RMS levels
pub fn test_level_equivalence(
    input_buffer: &[f32],
    output_buffer: &[f32],
    input_window_size: usize,
    output_window_size: usize,
    threshold: f32,
) {
    let input_chunks = input_buffer.chunks(input_window_size);
    let output_chunks = output_buffer.chunks(output_window_size);
    assert!(!input_buffer.is_empty());
    assert!(!output_buffer.is_empty());
    // assert!((input_chunks.len() as i32 - output_chunks.len() as i32).abs() < 2);
    for (input_chunk, output_chunk) in input_chunks.zip(output_chunks) {
        let input_level = util::rms_level(input_chunk);
        let output_level = util::rms_level(output_chunk);
        let diff = (input_level - output_level).abs();

        assert!(diff < threshold);
    }
}
