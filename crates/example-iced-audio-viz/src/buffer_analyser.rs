use atomic_queue::Queue;
use audio_processor_traits::{AudioBuffer, AudioProcessor};
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

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        for sample in data.slice().chunks(data.num_channels()) {
            self.buffer.push(sample[0]);
        }
    }
}
