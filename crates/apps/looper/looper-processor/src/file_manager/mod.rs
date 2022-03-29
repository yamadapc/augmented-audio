use std::path::{Path, PathBuf};

use audio_processor_file::file_io::AudioFileError;
use audio_processor_traits::{AudioProcessorSettings, VecAudioBuffer};

struct AudioClipModel {
    #[allow(dead_code)]
    path: PathBuf,
    #[allow(dead_code)]
    contents: VecAudioBuffer<f32>,
}

#[derive(Default)]
pub struct AudioClipManager {
    settings: AudioProcessorSettings,
    audio_clips: Vec<AudioClipModel>,
}

impl AudioClipManager {
    pub fn load_at_path(&mut self, path: &Path) -> Result<(), AudioFileError> {
        log::info!("Reading file at path {:?}", path);
        let mut audio_file =
            audio_processor_file::InMemoryAudioFile::from_path(path.to_str().unwrap())?;
        let audio_file = audio_file.read_into_vec_audio_buffer(&self.settings)?;
        self.audio_clips.push(AudioClipModel {
            path: path.into(),
            contents: audio_file,
        });
        Ok(())
    }
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
    }
}
