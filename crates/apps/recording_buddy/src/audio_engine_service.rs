use augmented::application::audio_processor_start;
use augmented::audio::processor::{AudioBuffer, AudioProcessor};
use std::sync::Arc;

pub struct AudioEngineService {}

impl Default for AudioEngineService {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioEngineService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(self: &Arc<Self>) {
        let processor = RecordingBuddyProcessor {};
        // TODO - this will leak memory
        std::mem::forget(audio_processor_start(processor));
    }
}

struct RecordingBuddyProcessor {}

impl AudioProcessor for RecordingBuddyProcessor {
    type SampleType = f32;

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        _data: &mut BufferType,
    ) {
    }
}
