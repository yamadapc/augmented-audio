use std::sync::atomic::{AtomicUsize, Ordering};

use audio_processor_traits::{AtomicF32, AudioBuffer, VecAudioBuffer};

pub struct ScratchPad {
    buffer: VecAudioBuffer<AtomicF32>,
    cursor: AtomicUsize,
}

impl ScratchPad {
    pub fn new(buffer: VecAudioBuffer<AtomicF32>) -> Self {
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

    pub fn buffer(&self) -> &VecAudioBuffer<AtomicF32> {
        &self.buffer
    }
}
