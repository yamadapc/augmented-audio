use atomic_queue::Queue;
use audio_processor_traits::{AudioBuffer, AudioProcessor, AudioProcessorSettings};
use basedrop::{Handle, Shared};
use std::time::Duration;

pub struct BufferAnalyserProcessor {
    buffer: Shared<Queue<f32>>,
    duration: Duration,
    current_position: usize,
}

impl BufferAnalyserProcessor {
    pub fn new(handle: &Handle) -> Self {
        BufferAnalyserProcessor {
            buffer: Shared::new(handle, Queue::new((5. * 4410.0) as usize)),
            duration: Duration::from_secs(5),
            current_position: 0,
        }
    }

    pub fn queue(&self) -> Shared<Queue<f32>> {
        self.buffer.clone()
    }
}

impl AudioProcessor for BufferAnalyserProcessor {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        // assert_eq!(settings.sample_rate(), 44100.0);
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        for sample_index in 0..data.num_samples() {
            if sample_index % 10 == 0 {
                self.buffer.push(*data.get(0, sample_index));
            }
        }
    }
}
