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
use actix::Actor;
use basedrop::Shared;

use audio_processor_standalone::standalone_processor::StandaloneOptions;
use audio_processor_standalone::{StandaloneHandles, StandaloneProcessorImpl};

use crate::audio::time_info_provider::HostCallback;
use crate::{MultiTrackLooper, MultiTrackLooperHandle};

enum AudioState {
    Standalone {
        handles: StandaloneHandles<StandaloneProcessorImpl<MultiTrackLooper>>,
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
