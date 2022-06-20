use actix::Actor;
use basedrop::Shared;

use audio_processor_standalone::standalone_processor::StandaloneOptions;
use audio_processor_standalone::{StandaloneHandles, StandaloneProcessorImpl};

use crate::audio::time_info_provider::HostCallback;
use crate::{MultiTrackLooper, MultiTrackLooperHandle};

enum AudioState {
    Standalone {
        handles: StandaloneHandles,
        options: StandaloneOptions,
    },
    Hosted(MultiTrackLooper),
}

pub enum AudioModeParams {
    Standalone,
    Hosted(Option<HostCallback>),
}

pub struct AudioStateController {
    handle: Shared<MultiTrackLooperHandle>,
    state: AudioState,
}

impl AudioStateController {
    pub fn new(audio_mode: AudioModeParams, processor: MultiTrackLooper) -> Self {
        let standalone_options = StandaloneOptions::default();
        let handle = processor.handle().clone();
        let state = match audio_mode {
            AudioModeParams::Standalone => setup_audio_state(standalone_options, processor),
            AudioModeParams::Hosted(_) => AudioState::Hosted(processor),
        };

        Self { handle, state }
    }

    /// Update audio options. Resets the audio threads and re-creates the MultiTrackLooper
    /// processor.
    pub fn set_options(&mut self, options: StandaloneOptions) {
        todo!("WHAT SHOULD THIS DO EXACTLY?")
    }
}

/// Set-up *stand-alone* audio state.
fn setup_audio_state(options: StandaloneOptions, processor: MultiTrackLooper) -> AudioState {
    let standalone_processor = StandaloneProcessorImpl::new_with(processor, options.clone());

    AudioState::Standalone {
        options,
        handles: audio_processor_standalone::standalone_start(
            standalone_processor,
            Some(audio_garbage_collector::handle()),
        ),
    }
}

impl Actor for AudioStateController {
    type Context = actix::Context<Self>;
}
