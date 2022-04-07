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
use std::time::Duration;

/// Similar to `crossbeam::utils::Backoff`, but for infrequent producers. Sleeps at exponentially
/// increasing timeouts when no elements appear. `LongBackoff::snooze` should be called **if there
/// was no work on this iteration**.
pub struct LongBackoff {
    backoff: crossbeam::utils::Backoff,
    iterations: usize,
}

impl LongBackoff {
    pub fn new() -> Self {
        Self {
            backoff: crossbeam::utils::Backoff::new(),
            iterations: 0,
        }
    }

    /// Snoozes with backoff for the first 100 iterations.
    ///
    /// Then starts sleeping from 1ms to 127ms with delays growing exponentially.
    ///
    /// Sleeps for at most 127ms
    pub fn snooze(&mut self) {
        if self.iterations < 100 {
            self.iterations += 1;
            self.backoff.snooze();
        } else {
            self.iterations += 1;
            let iteration = (self.iterations - 100).min(7); // 0-7
            let sleep_time = 2_u64.pow(iteration as u32); // 2^i
            std::thread::sleep(Duration::from_millis(sleep_time));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_backoff_pre_100() {
        let mut backoff = LongBackoff::new();
        for _i in 0..90 {
            backoff.snooze();
        }
    }

    #[test]
    fn test_backoff_past_100() {
        let mut backoff = LongBackoff::new();
        for _i in 0..110 {
            backoff.snooze();
        }
    }
}
