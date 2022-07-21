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

use std::sync::mpsc::{channel, Sender};

use basedrop::Handle;
use cpal::traits::DeviceTrait;
use cpal::{BufferSize, ChannelCount, SampleRate, StreamConfig};

use audio_processor_traits::{AudioProcessor, MidiEventHandler};

use crate::standalone_processor::{
    StandaloneAudioOnlyProcessor, StandaloneProcessor, StandaloneProcessorImpl,
};

use self::midi::{initialize_midi_host, MidiReference};
pub use self::mock_cpal::virtual_host::{VirtualHost, VirtualHostDevice, VirtualHostStream};
pub use self::options::AudioIOMode;

mod audio_thread;
mod error;
mod input_handling;
mod midi;
pub mod mock_cpal;
mod options;
mod output_handling;

/// Start an [`AudioProcessor`] / [`MidiEventHandler`] as a stand-alone cpal app and forward MIDI
/// messages received on all inputs to it.
///
/// Returns the [`cpal::Stream`]s and [`MidiHost`]. The audio-thread will keep running until these are
/// dropped.
pub fn audio_processor_start_with_midi<
    Processor: AudioProcessor<SampleType = f32> + MidiEventHandler + Send + 'static,
>(
    audio_processor: Processor,
    handle: &Handle,
) -> StandaloneHandles {
    let app = StandaloneProcessorImpl::new(audio_processor);
    standalone_start_with::<StandaloneProcessorImpl<Processor>, cpal::Host>(
        app,
        StandaloneStartOptions {
            handle: Some(handle.clone()),
            ..StandaloneStartOptions::<cpal::Host>::default()
        },
    )
}

/// Start an [`AudioProcessor`] as a stand-alone cpal app>
///
/// Returns the [`cpal::Stream`] streams. The audio-thread will keep running until these are dropped.
pub fn audio_processor_start<Processor: AudioProcessor<SampleType = f32> + Send + 'static>(
    audio_processor: Processor,
) -> StandaloneHandles {
    let app = StandaloneAudioOnlyProcessor::new(audio_processor, Default::default());
    standalone_start(app)
}

/// After negotiating options this struct is built with whatever devices and configuration used
/// for them.
#[derive(Debug)]
pub struct ResolvedStandaloneConfiguration {
    host: String,
    input_configuration: Option<IOConfiguration>,
    output_configuration: IOConfiguration,
}

impl ResolvedStandaloneConfiguration {
    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn input_configuration(&self) -> &Option<IOConfiguration> {
        &self.input_configuration
    }

    pub fn output_configuration(&self) -> &IOConfiguration {
        &self.output_configuration
    }
}

#[derive(Debug)]
pub struct IOConfiguration {
    name: String,
    buffer_size: BufferSize,
    sample_rate: SampleRate,
    num_channels: ChannelCount,
}

impl IOConfiguration {
    pub fn new(device: &impl DeviceTrait, config: &StreamConfig) -> IOConfiguration {
        IOConfiguration {
            name: device.name().unwrap(),
            sample_rate: config.sample_rate,
            buffer_size: config.buffer_size.clone(),
            num_channels: config.channels,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn buffer_size(&self) -> &BufferSize {
        &self.buffer_size
    }

    pub fn sample_rate(&self) -> SampleRate {
        self.sample_rate
    }

    pub fn num_channels(&self) -> ChannelCount {
        self.num_channels
    }
}

/// Handles to the CPAL streams and MIDI host. Playback will stop when these are dropped.
pub struct StandaloneHandles {
    configuration: ResolvedStandaloneConfiguration,
    // Handles contain a join handle with the thread, this might be used in the future.
    handle: Option<std::thread::JoinHandle<()>>,
    stop_signal_tx: Sender<()>,
    #[allow(unused)]
    midi_reference: MidiReference,
}

impl Drop for StandaloneHandles {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            let _ = self.stop_signal_tx.send(());
            handle.join().unwrap();
        }
        log::info!("Cleaning-up standalone handles");
    }
}

impl StandaloneHandles {
    pub fn configuration(&self) -> &ResolvedStandaloneConfiguration {
        &self.configuration
    }
}

pub struct StandaloneStartOptions<Host: cpal::traits::HostTrait> {
    pub host: Host,
    pub host_name: String,
    pub handle: Option<Handle>,
}

impl Default for StandaloneStartOptions<cpal::Host> {
    fn default() -> Self {
        let host = cpal::default_host();
        let host_name = host.id().name().to_string();
        Self {
            host,
            host_name,
            handle: Some(audio_garbage_collector::handle().clone()),
        }
    }
}

/// Start a processor using CPAL. Returns [`StandaloneHandles`] which can be used to take the
/// processor back and stop the stream.
///
/// Playback will stop when this value is dropped.
///
/// See [`standalone_start_with`] for more options
pub fn standalone_start<SP: StandaloneProcessor>(app: SP) -> StandaloneHandles {
    standalone_start_with::<SP, cpal::Host>(app, StandaloneStartOptions::<cpal::Host>::default())
}

/// Same as [`standalone_start`] but takes an options parameter.
pub fn standalone_start_with<
    SP: StandaloneProcessor,
    Host: cpal::traits::HostTrait + Send + 'static,
>(
    mut app: SP,
    options: StandaloneStartOptions<Host>,
) -> StandaloneHandles {
    let StandaloneStartOptions {
        handle,
        host,
        host_name,
    } = options;

    let _ = wisual_logger::try_init_from_env();

    let (midi_reference, midi_context) = initialize_midi_host(&mut app, handle.as_ref());

    let (configuration_tx, configuration_rx) = channel();
    let (stop_signal_tx, stop_signal_rx) = channel();
    // On iOS start takes over the calling thread, so this needs to be spawned in order for this
    // function to exit
    let handle = std::thread::Builder::new()
        .name(String::from("audio_thread"))
        .spawn(move || {
            let result = audio_thread::audio_thread_main(
                host,
                host_name,
                app,
                midi_context,
                configuration_tx,
                stop_signal_rx,
            );

            if let Err(err) = result {
                log::error!("Audio-thread failed with {}", err);
            }
        })
        .unwrap();

    let configuration = configuration_rx.recv().unwrap();
    log::info!("Received configuration {:?}", configuration);

    StandaloneHandles {
        configuration,
        handle: Some(handle),
        stop_signal_tx,
        midi_reference,
    }
}

/// Use [`VirtualHost`] on cfg(test), otherwise call `standalone_start`.
#[macro_export]
macro_rules! standalone_start_for_env {
    ($standalone_processor:ident) => {{
        #[cfg(test)]
        {
            log::warn!("Starting testing CPAL virtual host");
            standalone_start_with(
                $standalone_processor,
                StandaloneStartOptions {
                    host: VirtualHost::default(),
                    host_name: "Test Host".to_string(),
                    handle: Some(audio_garbage_collector::handle().clone()),
                },
            )
        }
        #[cfg(not(test))]
        {
            standalone_start($standalone_processor)
        }
    }};
}

#[macro_export]
macro_rules! generic_standalone_run {
    ($t: ident) => {
        match ::std::env::var("GUI") {
            Ok(value) if value == "true" => {
                use ::audio_processor_traits::parameters::{
                    AudioProcessorHandleProvider, AudioProcessorHandleRef,
                };
                let handle: AudioProcessorHandleRef =
                    AudioProcessorHandleProvider::generic_handle(&$t);
                let _audio_handles = ::audio_processor_standalone::audio_processor_start($t);
                ::audio_processor_standalone_gui::open(handle);
            }
            _ => {
                ::audio_processor_standalone::audio_processor_main($t);
            }
        }
    };
}

#[cfg(test)]
mod test {
    use audio_processor_traits::{BufferProcessor, NoopAudioProcessor};

    use super::*;

    #[test]
    fn test_standalone_start_and_stop_processor() {
        let _ = wisual_logger::try_init_from_env();
        let processor = BufferProcessor(NoopAudioProcessor::default());
        let processor = StandaloneAudioOnlyProcessor::new(processor, Default::default());
        let handles = standalone_start_for_env!(processor);
        drop(handles);
    }
}
