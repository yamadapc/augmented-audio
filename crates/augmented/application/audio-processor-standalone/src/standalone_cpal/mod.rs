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

use std::sync::mpsc::channel;

use basedrop::Handle;
use cpal::traits::DeviceTrait;
use cpal::{
    traits::StreamTrait, BufferSize, ChannelCount, Device, SampleRate, Stream, StreamConfig,
};

use audio_processor_traits::{AudioProcessor, AudioProcessorSettings, MidiEventHandler};

use crate::standalone_processor::{
    StandaloneAudioOnlyProcessor, StandaloneProcessor, StandaloneProcessorImpl,
};

use self::error::AudioThreadError;
use self::midi::{initialize_midi_host, MidiContext, MidiReference};

mod error;
mod input_handling;
mod midi;
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
    standalone_start(app, Some(handle))
}

/// Start an [`AudioProcessor`] as a stand-alone cpal app>
///
/// Returns the [`cpal::Stream`] streams. The audio-thread will keep running until these are dropped.
pub fn audio_processor_start<Processor: AudioProcessor<SampleType = f32> + Send + 'static>(
    audio_processor: Processor,
) -> StandaloneHandles {
    let app = StandaloneAudioOnlyProcessor::new(audio_processor, Default::default());
    standalone_start(app, None)
}

/// After negotiating options this struct is built with whatever devices and configuration used
/// for them.
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

pub struct IOConfiguration {
    name: String,
    buffer_size: BufferSize,
    sample_rate: SampleRate,
    num_channels: ChannelCount,
}

impl IOConfiguration {
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
    #[allow(unused)]
    handle: std::thread::JoinHandle<()>,
    #[allow(unused)]
    midi_reference: MidiReference,
}

impl Drop for StandaloneHandles {
    fn drop(&mut self) {
        self.handle.thread().unpark();
        log::info!("Cleaning-up standalone handles");
    }
}

impl StandaloneHandles {
    pub fn configuration(&self) -> &ResolvedStandaloneConfiguration {
        &self.configuration
    }
}

/// Start a processor using CPAL. Returns [`StandaloneHandles`] which can be used to take the
/// processor back and stop the stream.
///
/// Playback will stop when this value is dropped.
pub fn standalone_start<SP: StandaloneProcessor>(
    mut app: SP,
    handle: Option<&Handle>,
) -> StandaloneHandles {
    let _ = wisual_logger::try_init_from_env();

    let (midi_reference, midi_context) = initialize_midi_host(&mut app, handle);

    let (configuration_tx, configuration_rx) = channel();
    // On iOS start takes over the calling thread, so this needs to be spawned in order for this
    // function to exit
    let handle = std::thread::Builder::new()
        .name(String::from("audio_thread"))
        .spawn(move || {
            // Audio set-up
            let host = cpal::default_host();
            let host_name = host.id().name().to_string();
            log::info!("Using host: {}", host.id().name());
            let buffer_size = 512;
            let sample_rate = {
                #[cfg(not(target_os = "ios"))]
                {
                    44100
                }
                #[cfg(target_os = "ios")]
                {
                    48000
                }
            };

            let options = app.options();
            let accepts_input = options.accepts_input;
            let input_tuple = if accepts_input {
                Some(options::configure_input_device(
                    &host,
                    &options,
                    buffer_size,
                    sample_rate,
                ))
            } else {
                None
            };
            let (output_device, output_config) =
                options::configure_output_device(host, &options, buffer_size, sample_rate);

            let num_output_channels = output_config.channels.into();
            let num_input_channels = input_tuple
                .as_ref()
                .map(|(_, input_config)| input_config.channels.into())
                .unwrap_or(num_output_channels);

            let settings = AudioProcessorSettings::new(
                output_config.sample_rate.0 as f32,
                num_input_channels,
                num_output_channels,
                buffer_size,
            );
            app.processor().prepare(settings);

            configuration_tx
                .send(ResolvedStandaloneConfiguration {
                    host: host_name,
                    input_configuration: input_tuple.as_ref().map(|(input_device, config)| {
                        IOConfiguration {
                            name: input_device.name().unwrap(),
                            sample_rate: config.sample_rate.clone(),
                            buffer_size: config.buffer_size.clone(),
                            num_channels: config.channels,
                        }
                    }),
                    output_configuration: IOConfiguration {
                        name: output_device.name().unwrap(),
                        sample_rate: output_config.sample_rate.clone(),
                        buffer_size: output_config.buffer_size.clone(),
                        num_channels: output_config.channels.clone(),
                    },
                })
                .unwrap();

            let run_params = AudioThreadRunParams {
                io_hints: AudioThreadIOHints {
                    buffer_size,
                    num_output_channels,
                    num_input_channels,
                },
                cpal_streams: AudioThreadCPalStreams {
                    output_config,
                    input_tuple,
                    output_device,
                },
                midi_context,
            };

            audio_thread_run(run_params, app);
            std::thread::park();
        })
        .unwrap();

    let configuration = configuration_rx.recv().unwrap();

    StandaloneHandles {
        configuration,
        handle,
        midi_reference,
    }
}

/// At this point we have "negotiated" the nº of channels and buffer size.
/// These will be used in logic on the callbacks as well as to size our ringbuffer.
struct AudioThreadIOHints {
    buffer_size: usize,
    num_output_channels: usize,
    num_input_channels: usize,
}

/// Input and output audio streams.
struct AudioThreadCPalStreams {
    output_config: StreamConfig,
    input_tuple: Option<(Device, StreamConfig)>,
    output_device: Device,
}

struct AudioThreadRunParams {
    midi_context: Option<MidiContext>,
    io_hints: AudioThreadIOHints,
    cpal_streams: AudioThreadCPalStreams,
}

// Start this processor with given run parameters.
// The processor should be prepared at this point.
fn audio_thread_run(params: AudioThreadRunParams, app: impl StandaloneProcessor) {
    let AudioThreadRunParams {
        midi_context,
        io_hints,
        cpal_streams,
    } = params;
    let AudioThreadIOHints {
        buffer_size,
        num_output_channels,
        num_input_channels,
    } = io_hints;
    let AudioThreadCPalStreams {
        output_config,
        input_tuple,
        output_device,
    } = cpal_streams;

    let build_streams = move || -> Result<(Option<Stream>, Stream), AudioThreadError> {
        let buffer = ringbuf::RingBuffer::new((buffer_size * 10) as usize);
        let (producer, consumer) = buffer.split();
        let input_stream = input_handling::build_input_stream(input_tuple, producer)?;
        let output_stream = output_handling::build_output_stream(
            app,
            midi_context,
            num_output_channels,
            num_input_channels,
            consumer,
            output_device,
            output_config,
        )?;

        Ok((input_stream, output_stream))
    };

    match build_streams() {
        Ok((input_stream, output_stream)) => {
            log::info!("Audio streams starting on audio-thread");
            let play = || -> Result<(), AudioThreadError> {
                if let Some(input_stream) = &input_stream {
                    input_stream
                        .play()
                        .map_err(AudioThreadError::InputStreamError)?;
                }

                output_stream
                    .play()
                    .map_err(AudioThreadError::OutputStreamError)?;

                Ok(())
            };

            if let Err(err) = play() {
                log::error!("Audio-thread failed to start with {}", err);
                return;
            }

            log::info!("Audio streams started");
            std::thread::park();
        }
        Err(err) => {
            log::error!("Audio-thread failed to start with {}", err);
        }
    }
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
