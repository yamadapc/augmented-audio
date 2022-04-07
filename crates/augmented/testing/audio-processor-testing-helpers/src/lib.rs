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
pub use generators::*;
pub use util::rms_level;

pub mod charts;
mod generators;
mod util;

#[macro_export]
macro_rules! assert_f_eq {
    ($left:expr, $right:expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                assert!(
                    ((left_val - right_val).abs() as f32) < f32::EPSILON,
                    "left: {:?} not equal right: {:?}",
                    left_val,
                    right_val
                );
            }
        }
    }};
}

#[macro_export]
macro_rules! relative_path {
    ($path: expr) => {
        format!("{}/./{}", env!("CARGO_MANIFEST_DIR"), $path)
    };
}

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

        assert!(
            diff < threshold,
            "diff={} threshold={} input_level={} output_level={}",
            diff,
            threshold,
            input_level,
            output_level
        );
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_relative_path() {
        assert!(std::fs::read_to_string(relative_path!("src/lib.rs"))
            .unwrap()
            .contains("fn test_relative_path()"));
    }
}
