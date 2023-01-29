use crate::AudioProcessorSettings;

#[derive(Default)]
pub struct AudioContext {
    settings: AudioProcessorSettings,
}

impl AudioContext {
    pub fn settings(&self) -> &AudioProcessorSettings {
        &self.settings
    }
}

impl From<AudioProcessorSettings> for AudioContext {
    fn from(value: AudioProcessorSettings) -> Self {
        Self { settings: value }
    }
}
