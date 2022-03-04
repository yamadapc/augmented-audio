use audio_garbage_collector::{make_shared, Shared};
use audio_processor_traits::{AudioBuffer, AudioProcessor, AudioProcessorSettings};

use crate::{LooperProcessor, LooperProcessorHandle};

pub struct LooperId(usize);

pub struct MultiTrackLooperHandle {
    handles: Vec<Shared<LooperProcessorHandle>>,
}

impl MultiTrackLooperHandle {
    pub fn start_recording(&self, looper_id: LooperId) {
        self.handles[looper_id.0].start_recording();
    }
}

pub struct MultiTrackLooper {
    voices: Vec<LooperProcessor>,
    handle: Shared<MultiTrackLooperHandle>,
}

impl Default for MultiTrackLooper {
    fn default() -> Self {
        Self::new(3)
    }
}

impl MultiTrackLooper {
    fn new(num_voices: usize) -> Self {
        let voices: Vec<LooperProcessor> = (0..num_voices)
            .map(|_| LooperProcessor::default())
            .collect();
        let handle = make_shared(MultiTrackLooperHandle {
            handles: voices.iter().map(|voice| voice.handle().clone()).collect(),
        });

        Self { voices, handle }
    }
}

impl AudioProcessor for MultiTrackLooper {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        for voice in &mut self.voices {
            voice.prepare(settings);
        }
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        for voice in &mut self.voices {
            voice.process(data);
        }
    }
}
