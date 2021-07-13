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
        for sample_index in 0..data.num_samples() {
            if sample_index % 10 == 0 {
                self.buffer.push(*data.get(0, sample_index));
            }
        }
    }
}
