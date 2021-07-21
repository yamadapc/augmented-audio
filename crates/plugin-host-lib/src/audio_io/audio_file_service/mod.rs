use crate::processors::audio_file_processor::{
    default_read_audio_file, AudioFileError, AudioFileProcessor, AudioFileSettings,
};
use audio_processor_traits::AudioProcessorSettings;

pub fn probe_file(input_audio_path: &str) -> Result<AudioFileSettings, AudioFileError> {
    default_read_audio_file(input_audio_path).map(AudioFileSettings::new)
}

pub fn decode_and_prepare_processor(
    audio_processor_settings: AudioProcessorSettings,
    audio_file_settings: AudioFileSettings,
) -> AudioFileProcessor {
    let mut processor = AudioFileProcessor::new(audio_file_settings, audio_processor_settings);
    processor.prepare(audio_processor_settings);
    processor
}
