use atomic_queue::Queue;
use audio_processor_traits::{AudioBuffer, AudioProcessor, AudioProcessorSettings};
use basedrop::{Handle, Shared};

pub struct BufferAnalyserProcessor {
    buffer: Shared<Queue<f32>>,
}

impl BufferAnalyserProcessor {
    pub fn new(handle: &Handle) -> Self {
        BufferAnalyserProcessor {
            buffer: Shared::new(handle, Queue::new((5. * 4410.0) as usize)),
        }
    }

    pub fn queue(&self) -> Shared<Queue<f32>> {
        self.buffer.clone()
    }
}

impl AudioProcessor for BufferAnalyserProcessor {
    type SampleType = f32;

    fn prepare(&mut self, _settings: AudioProcessorSettings) {
        // assert_eq!(settings.sample_rate(), 44100.0);
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        for frame in data.slice().chunks(data.num_channels()) {
            self.buffer.push(frame[0]);
        }
    }
}
