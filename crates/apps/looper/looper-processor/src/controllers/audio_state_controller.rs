use actix::Actor;
use audio_processor_standalone::StandaloneHandles;

use crate::audio::time_info_provider::HostCallback;
use crate::MultiTrackLooper;

pub enum AudioState {
    Standalone(StandaloneHandles),
    Hosted(MultiTrackLooper),
}

pub enum AudioModeParams {
    Standalone,
    Hosted(Option<HostCallback>),
}

pub struct AudioStateController {
    state: AudioState,
}

impl AudioStateController {
    pub fn new(audio_mode: AudioModeParams, processor: MultiTrackLooper) -> Self {
        let state = match audio_mode {
            AudioModeParams::Standalone => {
                AudioState::Standalone(audio_processor_standalone::audio_processor_start_with_midi(
                    processor,
                    audio_garbage_collector::handle(),
                ))
            }
            AudioModeParams::Hosted(_) => AudioState::Hosted(processor),
        };

        Self { state }
    }
}

impl Actor for AudioStateController {
    type Context = actix::Context<Self>;
}
