// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
use std::path::{Path, PathBuf};
use std::time::Duration;

use actix::{Actor, Handler};
use basedrop::Shared;
use bytesize::ByteSize;

use audio_garbage_collector::make_shared;
use audio_processor_file::file_io::AudioFileError;
use audio_processor_file::OutputAudioFileProcessor;
use audio_processor_traits::{AudioBuffer, AudioProcessorSettings};

use crate::audio::processor::handle::{looper_clip_copy_to_vec_buffer, LooperClipRef};

pub struct AudioClipId(usize);

pub struct AudioClipModel {
    #[allow(unused)]
    id: AudioClipId,
    #[allow(unused)]
    path: PathBuf,
    contents: AudioBuffer<f32>,
}

impl AudioClipModel {
    pub fn contents(&self) -> &AudioBuffer<f32> {
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
        let mut sum: f32 = 0.0;
        for sample_index in 0..audio_file.num_samples() {
            let mono_sample = audio_file.get_mono(sample_index);
            sum += mono_sample.abs();
        }
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

pub fn write_looper_clip(settings: AudioProcessorSettings, clip_path: &Path, clip: &LooperClipRef) {
    log::info!("Writing audio into {:?}", clip_path);

    let mut output_processor =
        OutputAudioFileProcessor::from_path(settings, clip_path.to_str().unwrap());
    output_processor.prepare(settings);

    let mut clip_buffer = looper_clip_copy_to_vec_buffer(clip);
    if let Err(err) = output_processor.process(&mut clip_buffer) {
        log::error!("Failed to write file: {}", err);
    }
}

fn estimate_file_size<SampleType>(audio_file: &AudioBuffer<SampleType>) -> ByteSize {
    ByteSize::b(
        (audio_file.num_channels() * audio_file.num_samples() * std::mem::size_of::<SampleType>())
            as u64,
    )
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::{relative_path, rms_level};

    use actix_system_threads::ActorSystem;
    use audio_processor_traits::{AudioContext, AudioProcessor};

    use crate::audio::multi_track_looper::looper_voice::LooperVoice;
    use crate::audio::processor::handle::LooperState;
    use crate::controllers::events_controller::EventsController;
    use crate::controllers::load_project_controller::LoadContext;
    use crate::services::project_manager::{
        LoadLatestProjectMessage, ProjectManager, SaveProjectMessage,
    };
    use crate::{controllers, LooperOptions, MultiTrackLooper};

    use super::*;

    #[test]
    fn test_load_file_at_path() {
        wisual_logger::init_from_env();
        let mut manager = AudioClipManager::default();
        let test_file_path = PathBuf::from(relative_path!("../../../../input-files/1sec-sine.mp3"));
        manager.load_at_path(&test_file_path).unwrap();

        let test_file_path = PathBuf::from(relative_path!("../../../../input-files/bass.wav"));
        let clip = manager.load_at_path(&test_file_path).unwrap();
        let level = rms_level(clip.contents().channel(0));
        assert!(level > 0.1);
    }

    #[test]
    fn test_roundtrip_to_file() {
        wisual_logger::init_from_env();
        let data_path = tempdir::TempDir::new("looper_processor__audio_clip_manager").unwrap();

        let project_manager = ActorSystem::start(ProjectManager::new(data_path.path().into()));

        let mut input_buffer = AudioBuffer::empty();
        input_buffer.resize(2, 5);
        for channel in 0..2 {
            input_buffer.set(channel, 0, 0.1);
            input_buffer.set(channel, 1, 0.2);
            input_buffer.set(channel, 2, 0.3);
            input_buffer.set(channel, 3, 0.4);
            input_buffer.set(channel, 4, 0.5);
        }
        assert_eq!(2, input_buffer.num_channels());

        // Create mock looper with mock contents
        let mut looper = MultiTrackLooper::new(LooperOptions::default(), 1);
        let voice: &LooperVoice = &looper.handle().voices()[0];
        voice.looper().set_looper_buffer(&input_buffer);

        // Save its project
        let handle = looper.handle().clone();
        ActorSystem::current()
            .spawn_result({
                let project_manager = project_manager.clone();
                async move { project_manager.send(SaveProjectMessage { handle }).await }
            })
            .unwrap()
            .unwrap();

        // Reset audioclip manager so we know we're testing for a clean state
        let audio_clip_manager = ActorSystem::start(AudioClipManager::default());
        // Reset the looper handle so we know we're testing a clean state
        ActorSystem::current()
            .spawn_result({
                let project_manager = project_manager.clone();
                async move { project_manager.send(LoadLatestProjectMessage).await }
            })
            .unwrap()
            .unwrap();
        controllers::load_project_controller::load_and_hydrate_latest_project(LoadContext {
            handle: looper.handle().clone(),
            project_manager,
            audio_clip_manager,
            events_controller: ActorSystem::start(EventsController::default()),
        })
        .unwrap_or_else(|err| log::error!("Failed to load saved project: {}", err));

        // Test buffer is properly set
        let voice: &LooperVoice = &looper.handle().voices()[0];
        let buffer = voice.looper().looper_clip();
        let buffer = looper_clip_copy_to_vec_buffer(&buffer);
        assert_eq!(buffer.num_samples(), input_buffer.num_samples());
        assert_eq!(buffer.num_channels(), input_buffer.num_channels());
        assert_eq!(buffer, input_buffer);
        assert_eq!(voice.looper().state(), LooperState::Paused);

        // ======================================================================
        // Playback tests
        let settings = AudioProcessorSettings {
            block_size: buffer.num_samples(),
            ..Default::default()
        };
        let mut context = AudioContext::from(settings);
        looper.prepare(&mut context);

        // Test playback works
        looper.handle().voices()[0].looper().play();
        let mut buffer = AudioBuffer::empty();
        buffer.resize(2, input_buffer.num_samples());
        looper.process(&mut context, &mut buffer);
        assert_eq!(buffer.num_samples(), input_buffer.num_samples());
        assert_eq!(buffer.num_channels(), input_buffer.num_channels());
        assert_eq!(buffer, input_buffer);
    }
}
