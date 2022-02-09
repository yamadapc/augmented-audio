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
        let cursor = self.cursor.load(Ordering::Relaxed);
        let num_samples = self.buffer.num_samples();

        self.buffer.get(channel, cursor % num_samples).set(sample);
    }

    pub fn after_process(&self) {
        self.cursor.fetch_add(1, Ordering::Relaxed);
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
