use audio_processor_traits::{AudioBuffer, AudioProcessor, AudioProcessorSettings};
use basedrop::{Handle, Shared};
use std::time::Duration;
use vst::util::AtomicFloat;

pub struct VolumeAnalyserHandle {
    volume_left: AtomicFloat,
    volume_right: AtomicFloat,
}

impl VolumeAnalyserHandle {
    pub fn new(volume: f32) -> Self {
        VolumeAnalyserHandle {
            volume_left: AtomicFloat::new(volume),
            volume_right: AtomicFloat::new(volume),
        }
    }

    pub fn volume(&self) -> (f32, f32) {
        (self.volume_left.get(), self.volume_right.get())
    }
}

pub struct VolumeAnalyser {
    handle: Shared<VolumeAnalyserHandle>,
    internal_buffer: Vec<(f32, f32)>,
    duration: Duration,
    cursor: usize,
}

impl VolumeAnalyser {
    pub fn new(handle: &Handle, duration: Duration) -> Self {
        VolumeAnalyser {
            handle: Shared::new(handle, VolumeAnalyserHandle::new(0.0)),
            internal_buffer: Vec::new(),
            duration,
            cursor: 0,
        }
    }

    pub fn handle(&self) -> &Shared<VolumeAnalyserHandle> {
        &self.handle
    }
}

impl AudioProcessor for VolumeAnalyser {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.internal_buffer.resize(
            (self.duration.as_secs_f32() * settings.sample_rate()) as usize,
            (0.0, 0.0),
        );
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        for frame in data.slice().chunks(data.num_channels()) {
            self.internal_buffer[self.cursor] = (frame[0], frame[1]);

            self.cursor += 1;
            if self.cursor >= self.internal_buffer.len() {
                self.cursor = 0;
                let volume_left = self.internal_buffer.iter().map(|(l, _)| *l).sum::<f32>()
                    / (self.internal_buffer.len() * data.num_channels()) as f32;
                let volume_right = self.internal_buffer.iter().map(|(_, r)| *r).sum::<f32>()
                    / (self.internal_buffer.len() * data.num_channels()) as f32;
                self.handle.volume_left.set(volume_left);
                self.handle.volume_right.set(volume_right);
            }
        }
    }
}
