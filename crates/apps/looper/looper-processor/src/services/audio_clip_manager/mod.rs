use std::path::{Path, PathBuf};
use std::time::Duration;

use actix::{Actor, Handler, SyncContext};
use basedrop::Shared;
use bytesize::ByteSize;

use audio_garbage_collector::make_shared;
use audio_processor_file::file_io::AudioFileError;
use audio_processor_traits::{AudioBuffer, AudioProcessorSettings, VecAudioBuffer};

pub struct AudioClipId(usize);

pub struct AudioClipModel {
    #[allow(unused)]
    id: AudioClipId,
    #[allow(unused)]
    path: PathBuf,
    contents: VecAudioBuffer<f32>,
}

impl AudioClipModel {
    pub fn contents(&self) -> &impl AudioBuffer<SampleType = f32> {
        &self.contents
    }
}

pub type AudioClipModelRef = Shared<AudioClipModel>;

#[derive(Default)]
pub struct AudioClipManager {
    #[allow(dead_code)]
    settings: AudioProcessorSettings,
    #[allow(dead_code)]
    audio_clips: Vec<AudioClipModelRef>,
}

impl AudioClipManager {
    pub fn load_at_path(&mut self, path: &Path) -> Result<AudioClipModelRef, AudioFileError> {
        log::info!("Reading file at path {:?}", path);
        let mut audio_file =
            audio_processor_file::InMemoryAudioFile::from_path(path.to_str().unwrap())?;
        let audio_file = audio_file.read_into_vec_audio_buffer(&self.settings)?;
        let byte_size = estimate_file_size(&audio_file);
        let duration =
            Duration::from_secs_f32(audio_file.num_samples() as f32 / self.settings.sample_rate());
        log::info!(
            "File takes-up ~{} of memory, duration={:?}",
            byte_size,
            duration
        );

        let clip_model = make_shared(AudioClipModel {
            id: AudioClipId(self.audio_clips.len()),
            path: path.into(),
            contents: audio_file,
        });
        self.audio_clips.push(clip_model.clone());
        Ok(clip_model)
    }
}

impl Actor for AudioClipManager {
    type Context = SyncContext<Self>;
}

#[derive(actix::Message)]
#[rtype(result = "Result<AudioClipModelRef, AudioFileError>")]
pub(crate) struct LoadClipMessage {
    pub(crate) path: PathBuf,
}

impl Handler<LoadClipMessage> for AudioClipManager {
    type Result = Result<AudioClipModelRef, AudioFileError>;

    fn handle(&mut self, msg: LoadClipMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.load_at_path(&msg.path)
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
