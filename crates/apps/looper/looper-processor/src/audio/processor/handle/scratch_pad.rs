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
use std::sync::atomic::{AtomicUsize, Ordering};

use audio_processor_traits::{AtomicF32, AudioBuffer};

pub struct ScratchPad {
    buffer: AudioBuffer<AtomicF32>,
    cursor: AtomicUsize,
}

impl ScratchPad {
    pub fn new(buffer: AudioBuffer<AtomicF32>) -> Self {
        ScratchPad {
            buffer,
            cursor: AtomicUsize::new(0),
        }
    }

    pub fn set(&self, channel: usize, sample: f32) {
        let num_samples = self.buffer.num_samples();
        if num_samples == 0 {
            return;
        }

        let cursor = self.cursor.load(Ordering::Relaxed);
        self.buffer.get(channel, cursor).set(sample);
    }

    pub fn after_process(&self) {
        let mut new_cursor = self.cursor.load(Ordering::Relaxed) + 1;
        if new_cursor >= self.buffer.num_samples() {
            new_cursor = 0;
        }
        self.cursor.store(new_cursor, Ordering::Relaxed);
    }

    pub fn cursor(&self) -> usize {
        self.cursor.load(Ordering::Relaxed)
    }

    pub fn max_len(&self) -> usize {
        self.buffer.num_samples()
    }

    pub fn buffer(&self) -> &AudioBuffer<AtomicF32> {
        &self.buffer
    }
}

#[cfg(test)]
mod test {
    use audio_processor_traits::AudioBuffer;
    use augmented_atomics::AtomicF32;

    use super::*;

    #[test]
    fn test_scratch_pad_new() {
        let mut buffer = AudioBuffer::empty();
        buffer.resize_with(2, 10, || AtomicF32::new(0.0));
        let _scratch_pad = ScratchPad::new(buffer);
    }

    #[test]
    fn test_after_process_will_increment_cursor() {
        let mut buffer = AudioBuffer::empty();
        buffer.resize_with(2, 10, || AtomicF32::new(0.0));
        let scratch_pad = ScratchPad::new(buffer);
        assert_eq!(scratch_pad.cursor(), 0);
        scratch_pad.after_process();
        assert_eq!(scratch_pad.cursor(), 1);
        for _i in 0..12 {
            scratch_pad.after_process()
        }
        assert_eq!(scratch_pad.cursor(), 3);
    }

    #[test]
    fn test_set_will_update_the_buffer_at_the_current_cursor() {
        let mut buffer = AudioBuffer::empty();
        buffer.resize_with(2, 10, || AtomicF32::new(0.0));
        let scratch_pad = ScratchPad::new(buffer);
        scratch_pad.set(0, 10.0);
        assert_eq!(scratch_pad.buffer().get(0, 0).get(), 10.0);
        scratch_pad.after_process();
        scratch_pad.set(1, 15.0);
        assert_eq!(scratch_pad.buffer().get(1, 1).get(), 15.0);
    }

    #[test]
    fn test_scratch_max_len_is_the_storage_size() {
        let mut buffer = AudioBuffer::empty();
        buffer.resize_with(2, 10, || AtomicF32::new(0.0));
        let scratch_pad = ScratchPad::new(buffer);
        assert_eq!(scratch_pad.max_len(), 10);
    }
}
