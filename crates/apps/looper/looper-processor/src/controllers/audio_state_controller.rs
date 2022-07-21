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
use actix::{Actor, Handler};
use basedrop::Shared;
use cpal::traits::{DeviceTrait, HostTrait};

use audio_processor_standalone::standalone_cpal::AudioIOMode;
use audio_processor_standalone::standalone_processor::StandaloneOptions;
use audio_processor_standalone::{StandaloneHandles, StandaloneProcessorImpl};

use crate::audio::time_info_provider::HostCallback;
use crate::{MultiTrackLooper, MultiTrackLooperHandle};

enum AudioState {
    Standalone {
        #[allow(unused)]
        handles: StandaloneHandles,
        #[allow(unused)]
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
    state: Option<AudioState>,
}

impl AudioStateController {
    pub fn new(audio_mode: AudioModeParams, processor: MultiTrackLooper) -> Self {
        let standalone_options = StandaloneOptions::default();
        let handle = processor.handle().clone();
        let state = match audio_mode {
            AudioModeParams::Standalone => setup_audio_state(standalone_options, processor),
            AudioModeParams::Hosted(_) => AudioState::Hosted(processor),
        };

        Self {
            handle,
            state: Some(state),
        }
    }

    /// Update audio options. Resets the audio threads and re-creates the MultiTrackLooper
    /// processor.
    pub fn set_options(&mut self, options: StandaloneOptions) {
        let current_options = self.get_options().unwrap_or_default();
        if options.input_device == current_options.input_device
            && options.output_device == current_options.output_device
        {
            log::warn!(
                "Ignoring noop IO options update input={:?} output={:?}",
                options.input_device,
                options.output_device
            );
            return;
        }

        let state = self.state.take();
        drop(state);
        let processor = MultiTrackLooper::from_handle(Default::default(), 8, self.handle.clone());
        self.state = Some(setup_audio_state(options, processor));
        log::info!("=== Restarted audio-thread successfully ======\n");
    }

    fn get_options(&mut self) -> Option<StandaloneOptions> {
        let options = self.state.as_ref().and_then(|state| match state {
            AudioState::Standalone { options, .. } => Some(options.clone()),
            _ => None,
        });
        options
    }
}

/// Set-up *stand-alone* audio state.
fn setup_audio_state(options: StandaloneOptions, processor: MultiTrackLooper) -> AudioState {
    let standalone_processor = StandaloneProcessorImpl::new_with(processor, options);
    let handles = audio_processor_standalone::standalone_cpal::standalone_start_for_env!(
        standalone_processor
    );
    let options = StandaloneOptions {
        accepts_input: true,
        input_device: handles
            .configuration()
            .input_configuration()
            .as_ref()
            .map(|config| config.name().to_string()),
        output_device: Some(
            handles
                .configuration()
                .output_configuration()
                .name()
                .to_string(),
        ),
        handle: None,
    };

    AudioState::Standalone { options, handles }
}

impl Actor for AudioStateController {
    type Context = actix::Context<Self>;
}

#[derive(actix::Message)]
#[rtype(result = "()")]
pub enum SetOptions {
    StandaloneOptions(StandaloneOptions),
    InputDevice(String),
    OutputDevice(String),
}

impl Handler<SetOptions> for AudioStateController {
    type Result = ();

    fn handle(&mut self, msg: SetOptions, _ctx: &mut Self::Context) -> Self::Result {
        let options = self.get_options().unwrap_or_default();
        match msg {
            SetOptions::StandaloneOptions(options) => {
                self.set_options(options);
            }
            SetOptions::InputDevice(input_device) => self.set_options(StandaloneOptions {
                input_device: Some(input_device),
                ..options
            }),
            SetOptions::OutputDevice(output_device) => self.set_options(StandaloneOptions {
                output_device: Some(output_device),
                ..options
            }),
        }
    }
}

#[derive(actix::Message)]
#[rtype(result = "Option<StandaloneOptions>")]
pub struct GetOptions {}

impl Handler<GetOptions> for AudioStateController {
    type Result = Option<StandaloneOptions>;
    fn handle(&mut self, _msg: GetOptions, _ctx: &mut Self::Context) -> Self::Result {
        self.get_options()
    }
}

pub struct AudioDevice {
    pub name: String,
}

#[derive(actix::Message)]
#[rtype(result = "anyhow::Result<Vec<AudioDevice>>")]
pub struct ListDevices {
    pub mode: AudioIOMode,
}

impl Handler<ListDevices> for AudioStateController {
    type Result = anyhow::Result<Vec<AudioDevice>>;

    fn handle(&mut self, msg: ListDevices, _ctx: &mut Self::Context) -> Self::Result {
        let mode = msg.mode;
        let host = cpal::default_host();
        let devices = match mode {
            AudioIOMode::Input => host.input_devices()?,
            AudioIOMode::Output => host.output_devices()?,
        };
        build_domain_model(devices)
    }
}

fn build_domain_model(
    devices: impl Iterator<Item = cpal::Device>,
) -> anyhow::Result<Vec<AudioDevice>> {
    let mut result = vec![];

    for device in devices {
        let name = device.name()?;
        result.push(AudioDevice { name });
    }

    Ok(result)
}
