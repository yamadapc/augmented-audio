use audio_processor_traits::AudioProcessorSettings;

use crate::processors::audio_file_processor::file_io::{default_read_audio_file, AudioFileError};
use crate::processors::audio_file_processor::{AudioFileProcessor, AudioFileSettings};

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

#[cfg(test)]
mod test {
    use audio_processor_traits::AudioProcessorSettings;

    use super::{decode_and_prepare_processor, probe_file};

    #[test]
    fn test_smoke_probe_file() {
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let _probe = probe_file(&*format!(
            "{}/../../../../input-files/C3-loop.mp3",
            crate_dir
        ))
        .unwrap();
    }

    #[test]
    fn test_smoke_decode_and_prepare_processor() {
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let probe = probe_file(&*format!(
            "{}/../../../../input-files/C3-loop.mp3",
            crate_dir
        ))
        .unwrap();
        let _processor = decode_and_prepare_processor(AudioProcessorSettings::default(), probe);
    }
}
