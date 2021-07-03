use vst::util::AtomicFloat;

use audio_processor_traits::{AudioBuffer, AudioProcessor, InterleavedAudioBuffer};
use circular_data_structures::CircularVec;

pub struct VolumeMeterProcessor {
    volume_left: AtomicFloat,
    volume_right: AtomicFloat,
    current_index: usize,
    buffer_duration_samples: usize,
    left_buffer: CircularVec<f32>,
    right_buffer: CircularVec<f32>,
}

impl VolumeMeterProcessor {
    pub fn new() -> Self {
        VolumeMeterProcessor {
            volume_left: AtomicFloat::new(0.0),
            volume_right: AtomicFloat::new(0.0),
            current_index: 0,
            buffer_duration_samples: 512 * 4,
            left_buffer: CircularVec::with_size(512 * 4, 0.0),
            right_buffer: CircularVec::with_size(512 * 4, 0.0),
        }
    }

    pub fn current_volume(&self) -> (f32, f32) {
        (self.volume_left.get(), self.volume_right.get())
    }

    pub fn calculate_rms(buffer: &CircularVec<f32>) -> f32 {
        let mut sum = 0.0;
        for i in 0..buffer.len() {
            let v = buffer[i];
            sum += v.abs();
        }
        sum / buffer.len() as f32
    }
}

impl AudioProcessor<InterleavedAudioBuffer<'_, f32>> for VolumeMeterProcessor {
    fn process(&mut self, data: &mut InterleavedAudioBuffer<f32>) {
        for frame_index in 0..data.num_samples() {
            self.left_buffer[self.current_index] = *data.get(0, frame_index);
            self.right_buffer[self.current_index] = *data.get(1, frame_index);

            if self.current_index >= self.buffer_duration_samples {
                self.current_index = 0;
                self.volume_right
                    .set(Self::calculate_rms(&self.right_buffer));
                self.volume_left.set(Self::calculate_rms(&self.left_buffer));
            } else {
                self.current_index += 1;
            }
        }
    }
}
