use audio_processor_traits::AudioProcessor;
use circular_data_structures::CircularVec;
use vst::util::AtomicFloat;

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

impl AudioProcessor for VolumeMeterProcessor {
    fn process(&mut self, data: &mut [f32]) {
        for frame in data.chunks_mut(2) {
            self.left_buffer[self.current_index] = frame[0];
            self.right_buffer[self.current_index] = frame[1];

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
