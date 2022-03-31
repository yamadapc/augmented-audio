use std::path::{Path, PathBuf};

use bytesize::ByteSize;

use audio_processor_file::file_io::AudioFileError;
use audio_processor_traits::{AudioBuffer, AudioProcessorSettings, VecAudioBuffer};

pub struct AudioClipModel {
    #[allow(dead_code)]
    path: PathBuf,
    #[allow(dead_code)]
    contents: VecAudioBuffer<f32>,
}

#[derive(Default)]
pub struct AudioClipManager {
    #[allow(dead_code)]
    settings: AudioProcessorSettings,
    #[allow(dead_code)]
    audio_clips: Vec<AudioClipModel>,
}

impl AudioClipManager {
    #[allow(dead_code)]
    pub fn load_at_path(&mut self, path: &Path) -> Result<(), AudioFileError> {
        log::info!("Reading file at path {:?}", path);
        let mut audio_file =
            audio_processor_file::InMemoryAudioFile::from_path(path.to_str().unwrap())?;
        let audio_file = audio_file.read_into_vec_audio_buffer(&self.settings)?;
        let byte_size = estimate_file_size(&audio_file);
        log::info!("File takes-up ~{} of memory", byte_size);
        self.audio_clips.push(AudioClipModel {
            path: path.into(),
            contents: audio_file,
        });
        Ok(())
    }
}

fn estimate_file_size<Buffer: AudioBuffer>(audio_file: &Buffer) -> ByteSize {
    let byte_size = ByteSize::b(
        (audio_file.num_channels()
            * audio_file.num_samples()
            * std::mem::size_of::<Buffer::SampleType>()) as u64,
    );
    byte_size
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::relative_path;

    use super::*;

    #[test]
    fn test_load_file_at_path() {
        wisual_logger::init_from_env();
        let mut manager = AudioClipManager::default();
        let test_file_path = PathBuf::from(relative_path!("../../../../input-files/1sec-sine.mp3"));
        manager.load_at_path(&test_file_path).unwrap();

        let test_file_path = PathBuf::from(relative_path!("../../../../input-files/bass.wav"));
        manager.load_at_path(&test_file_path).unwrap();
    }
}
