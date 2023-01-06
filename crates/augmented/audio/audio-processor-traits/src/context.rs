use crate::AudioProcessorSettings;

#[derive(Default)]
pub struct AudioContext {
    settings: AudioProcessorSettings,
}

impl From<AudioProcessorSettings> for AudioContext {
    fn from(value: AudioProcessorSettings) -> Self {
        Self { settings: value }
    }
}
