use rayon::prelude::*;
use symphonia::core::audio::Signal;
use symphonia::{core::audio::AudioBuffer as SymphoniaAudioBuffer, core::probe::ProbeResult};

use crate::processors::audio_file_processor::file_io::{
    default_read_audio_file, read_file_contents, AudioFileError,
};

// TODO: Add metadata & contents
// * Length
// * Metadata
// * Content buffer
pub struct AudioFile {
    probe: ProbeResult,
    contents: SymphoniaAudioBuffer<f32>,
    rms_snapshot: Vec<f32>,
}

trait AudioFileService {
    fn load_file(&self, input_audio_path: &str) -> Result<AudioFile, AudioFileError>;
}

pub struct AudioFileServiceImpl {}

impl Default for AudioFileServiceImpl {
    fn default() -> Self {
        Self {}
    }
}

impl AudioFileService for AudioFileServiceImpl {
    fn load_file(&self, input_audio_path: &str) -> Result<AudioFile, AudioFileError> {
        let mut probe = default_read_audio_file(input_audio_path)?;
        let contents = read_file_contents(&mut probe)?;
        let rms_snapshot = Self::calculate_rms_snapshot(&contents);
        Ok(AudioFile {
            probe,
            contents,
            rms_snapshot,
        })
    }
}

impl AudioFileServiceImpl {
    fn calculate_rms_snapshot(contents: &SymphoniaAudioBuffer<f32>) -> Vec<f32> {
        let converted_channels: Vec<Vec<f32>> = (0..contents.spec().channels.count())
            .into_par_iter()
            .map(|channel_number| contents.chan(channel_number).into())
            .collect();

        let mut buffer: Vec<f32> = vec![];
        buffer.resize(converted_channels[0].len(), 0.0);
        for channel in converted_channels {
            for (i, sample) in channel.iter().enumerate() {
                buffer[i] += sample;
            }
        }

        buffer
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_input_file() {
        let audio_file_service = AudioFileServiceImpl::default();
        let file_path = format!(
            "{}{}",
            env!("CARGO_MANIFEST_DIR"),
            "/../../../../input-files/synthetizer-loop.mp3"
        );
        let audio_file = audio_file_service.load_file(&file_path).unwrap();

        assert!(audio_file.contents.spec().channels.count() > 0);
        assert!(audio_file.rms_snapshot.len() > 0);
        // let file_path = format!(
        //     "{}{}",
        //     env!("CARGO_MANIFEST_DIR"),
        //     "/src/audio_io/audio_file_service/mod.rs"
        // );
        // audio_processor_testing_helpers::charts::draw_vec_chart(
        //     &file_path,
        //     "audio-file-chart",
        //     audio_file.rms_snapshot,
        // );
    }
}
