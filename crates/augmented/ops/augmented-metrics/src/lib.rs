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
//! Small helpers for augmented applications.
//!
//! * [`audio_processor_metrics`] exposes structs/functions for tracking CPU usage on the
//!   audio-thread
use std::time::Instant;

pub mod audio_processor_metrics;

/// Log duration of a function
pub fn time<T>(label: &str, body: impl FnOnce() -> T) -> T {
    let start = Instant::now();
    let result = body();
    log::info!("{} duration={}ms", label, start.elapsed().as_millis());
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_time() {
        let result = time("test_time", || 10_i32.pow(2));
        assert_eq!(result, 100);
    }
}
