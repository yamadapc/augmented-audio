use vst::util::AtomicFloat;

use audio_garbage_collector::{Handle, Shared};
use audio_processor_traits::{AudioBuffer, AudioProcessor, AudioProcessorSettings};
use circular_data_structures::CircularVec;
use std::time::Duration;

pub struct VolumeMeterProcessorHandle {
    pub volume_left: AtomicFloat,
    pub volume_right: AtomicFloat,
    pub peak_left: AtomicFloat,
    pub peak_right: AtomicFloat,
}

// TODO - this is really inefficient
pub struct VolumeMeterProcessor {
    handle: Shared<VolumeMeterProcessorHandle>,
    current_index: usize,
    buffer_duration: Duration,
    buffer_duration_samples: usize,
    left_buffer: CircularVec<f32>,
    right_buffer: CircularVec<f32>,
}

impl VolumeMeterProcessor {
    pub fn new(handle: &Handle) -> Self {
        VolumeMeterProcessor {
            handle: Shared::new(
                handle,
                VolumeMeterProcessorHandle {
                    volume_left: AtomicFloat::new(0.0),
                    volume_right: AtomicFloat::new(0.0),
                    peak_left: AtomicFloat::new(0.0),
                    peak_right: AtomicFloat::new(0.0),
                },
            ),
            current_index: 0,
            buffer_duration: Duration::from_millis(20),
            buffer_duration_samples: 512 * 4,
            left_buffer: CircularVec::with_size(512 * 4, 0.0),
            right_buffer: CircularVec::with_size(512 * 4, 0.0),
        }
    }

    pub fn handle(&self) -> &Shared<VolumeMeterProcessorHandle> {
        &self.handle
    }

    pub fn current_volume(&self) -> (f32, f32) {
        (
            self.handle.volume_left.get(),
            self.handle.volume_right.get(),
        )
    }

    fn calculate_rms(buffer: &CircularVec<f32>) -> f32 {
        let mut sum = 0.0;
        for i in 0..buffer.len() {
            let v = buffer[i];
            sum += v.abs();
        }
        sum / buffer.len() as f32
    }

    // TODO - Would it be faster to do all in one loop? Measure.
    fn calculate_peak(buffer: &CircularVec<f32>) -> f32 {
        let mut peak: f32 = 0.0;
        for i in 0..buffer.len() {
            let v = buffer[i];
            peak = peak.max(v.abs());
        }
        peak
    }
}

impl AudioProcessor for VolumeMeterProcessor {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        let duration_samples =
            (self.buffer_duration.as_secs_f32() * settings.sample_rate()) as usize;
        self.buffer_duration_samples = duration_samples;
        self.left_buffer.resize(duration_samples, 0.0);
        self.right_buffer.resize(duration_samples, 0.0);
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        for frame_index in 0..data.num_samples() {
            self.left_buffer[self.current_index] = *data.get(0, frame_index);
            self.right_buffer[self.current_index] = *data.get(1, frame_index);

            if self.current_index >= self.buffer_duration_samples {
                self.current_index = 0;
                self.handle
                    .volume_right
                    .set(Self::calculate_rms(&self.right_buffer));
                self.handle
                    .peak_right
                    .set(Self::calculate_peak(&self.right_buffer));
                self.handle
                    .volume_left
                    .set(Self::calculate_rms(&self.left_buffer));
                self.handle
                    .peak_left
                    .set(Self::calculate_peak(&self.left_buffer));
            } else {
                self.current_index += 1;
            }
        }
    }
}
