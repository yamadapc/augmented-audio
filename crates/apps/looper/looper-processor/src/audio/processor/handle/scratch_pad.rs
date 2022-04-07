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

#[cfg(test)]
mod test {
    use audio_processor_traits::{AudioBuffer, OwnedAudioBuffer, VecAudioBuffer};
    use augmented_atomics::AtomicF32;

    use super::*;

    #[test]
    fn test_scratch_pad_new() {
        let mut buffer = VecAudioBuffer::new();
        buffer.resize(2, 10, AtomicF32::new(0.0));
        let _scratch_pad = ScratchPad::new(buffer);
    }

    #[test]
    fn test_after_process_will_increment_cursor() {
        let mut buffer = VecAudioBuffer::new();
        buffer.resize(2, 10, AtomicF32::new(0.0));
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
        let mut buffer = VecAudioBuffer::new();
        buffer.resize(2, 10, AtomicF32::new(0.0));
        let scratch_pad = ScratchPad::new(buffer);
        scratch_pad.set(0, 10.0);
        assert_eq!(scratch_pad.buffer().get(0, 0).get(), 10.0);
        scratch_pad.after_process();
        scratch_pad.set(1, 15.0);
        assert_eq!(scratch_pad.buffer().get(1, 1).get(), 15.0);
    }

    #[test]
    fn test_scratch_max_len_is_the_storage_size() {
        let mut buffer = VecAudioBuffer::new();
        buffer.resize(2, 10, AtomicF32::new(0.0));
        let scratch_pad = ScratchPad::new(buffer);
        assert_eq!(scratch_pad.max_len(), 10);
    }
}
