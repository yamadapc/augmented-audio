use std::path::{Path, PathBuf};
use std::time::Duration;

use actix::{Actor, Handler};
use basedrop::Shared;
use bytesize::ByteSize;

use audio_garbage_collector::make_shared;
use audio_processor_file::file_io::AudioFileError;
use audio_processor_file::OutputAudioFileProcessor;
use audio_processor_traits::{AudioBuffer, AudioProcessorSettings, VecAudioBuffer};

use crate::audio::processor::handle::{looper_clip_copy_to_vec_buffer, LooperClipRef};

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
        let sum: f32 = audio_file
            .frames()
            .map(|frame| frame.iter().map(|f| f.abs()).sum::<f32>())
            .sum();
        let rms = sum / audio_file.num_samples() as f32;
        log::info!("RMS level rms={}", rms);

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
    type Context = actix::Context<Self>;
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

pub fn write_looper_clip(
    settings: AudioProcessorSettings,
    clip_path: &PathBuf,
    clip: &LooperClipRef,
) {
    log::info!("Writing audio into {:?}", clip_path);

    let mut output_processor =
        OutputAudioFileProcessor::from_path(settings, clip_path.to_str().unwrap());
    output_processor.prepare(settings);

    let mut clip_buffer = looper_clip_copy_to_vec_buffer(&clip);
    output_processor.process(clip_buffer.slice_mut());
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
    use audio_processor_testing_helpers::{relative_path, rms_level};

    use actix_system_threads::ActorSystemThread;

    use crate::audio::multi_track_looper::looper_voice::LooperVoice;
    use crate::services::project_manager::{
        LoadLatestProjectMessage, ProjectManager, SaveProjectMessage,
    };
    use crate::MultiTrackLooper;

    use super::*;

    #[test]
    fn test_load_file_at_path() {
        wisual_logger::init_from_env();
        let mut manager = AudioClipManager::default();
        let test_file_path = PathBuf::from(relative_path!("../../../../input-files/1sec-sine.mp3"));
        manager.load_at_path(&test_file_path).unwrap();

        let test_file_path = PathBuf::from(relative_path!("../../../../input-files/bass.wav"));
        let clip = manager.load_at_path(&test_file_path).unwrap();
        let level = rms_level(&clip.contents().slice());
        assert!(level > 0.1);
    }

    #[test]
    fn test_roundtrip_to_file() {
        wisual_logger::init_from_env();
        let data_path = tempdir::TempDir::new("looper_processor__audio_clip_manager").unwrap();

        let project_manager =
            ActorSystemThread::start(ProjectManager::new(data_path.path().into()));

        let mut input_buffer = VecAudioBuffer::empty_with(2, 5, 0.0);
        input_buffer.set(0, 0, 0.1);
        input_buffer.set(0, 1, 0.2);
        input_buffer.set(0, 2, 0.3);
        input_buffer.set(0, 3, 0.4);
        input_buffer.set(0, 4, 0.5);
        assert_eq!(2, input_buffer.num_channels());

        // Create mock looper with mock contents
        let looper = MultiTrackLooper::default();
        let voice: &LooperVoice = &looper.handle().voices()[0];
        voice.looper().set_looper_buffer(&input_buffer);

        // Save its project
        let handle = looper.handle().clone();
        ActorSystemThread::current()
            .spawn_result({
                let project_manager = project_manager.clone();
                async move { project_manager.send(SaveProjectMessage { handle }).await }
            })
            .unwrap()
            .unwrap();

        // Reset audioclip manager so we know we're testing for a clean state
        let manager = ActorSystemThread::start(AudioClipManager::default());
        // Reset the looper handle so we know we're testing a clean state
        ActorSystemThread::current()
            .spawn_result({
                let project_manager = project_manager.clone();
                async move { project_manager.send(LoadLatestProjectMessage).await }
            })
            .unwrap()
            .unwrap();
        crate::controllers::load_project_controller::load_and_hydrate_latest_project(
            &looper.handle(),
            &project_manager,
            &manager,
        )
        .unwrap();

        let voice: &LooperVoice = &looper.handle().voices()[0];
        let buffer = voice.looper().looper_clip();
        let buffer = looper_clip_copy_to_vec_buffer(&buffer);
        assert_eq!(buffer.num_samples(), input_buffer.num_samples());
        assert_eq!(buffer.num_channels(), input_buffer.num_channels());
        assert_eq!(buffer.slice(), input_buffer.slice());
    }
}
