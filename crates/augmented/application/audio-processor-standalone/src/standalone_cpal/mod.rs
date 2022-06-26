use std::sync::mpsc;
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
use basedrop::Handle;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BufferSize, Device, Host, SampleRate, Stream, StreamConfig,
};
use ringbuf::{Consumer, Producer};

use audio_processor_traits::{
    AudioBuffer, AudioProcessor, AudioProcessorSettings, InterleavedAudioBuffer, MidiEventHandler,
};
use midi::{flush_midi_events, initialize_midi_host, MidiContext, MidiReference};

use crate::standalone_processor::{
    StandaloneAudioOnlyProcessor, StandaloneOptions, StandaloneProcessor, StandaloneProcessorImpl,
};

mod midi;

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

/// Handles to the CPAL streams and MIDI host. Playback will stop when these are dropped.
pub struct StandaloneHandles {
    // Handles contain a join handle with the thread, this might be used in the future.
    #[allow(unused)]
    handle: std::thread::JoinHandle<()>,
    #[allow(unused)]
    midi_reference: MidiReference,
}

/// Start a processor using CPAL. Returns [`StandaloneHandles`] which can be used to take the
/// processor back and stop the stream.
///
/// Playback will stop when this value is dropped.
pub fn standalone_start(
    mut app: impl StandaloneProcessor,
    handle: Option<&Handle>,
) -> StandaloneHandles {
    let _ = wisual_logger::try_init_from_env();

    let (midi_reference, midi_context) = initialize_midi_host(&mut app, handle);

    // On iOS start takes over the calling thread, so this needs to be spawned in order for this
    // function to exit
    let handle = std::thread::Builder::new()
        .name(String::from("audio_thread"))
        .spawn(move || {
            // Audio set-up
            let host = cpal::default_host();
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
                Some(configure_input_device(
                    &host,
                    &options,
                    buffer_size,
                    sample_rate,
                ))
            } else {
                None
            };
            let (output_device, output_config) =
                configure_output_device(host, &options, buffer_size, sample_rate);

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
        })
        .unwrap();

    StandaloneHandles {
        handle,
        midi_reference,
    }
}

#[derive(thiserror::Error, Debug)]
#[allow(clippy::enum_variant_names)]
enum AudioThreadError {
    #[error("Failed to configure input stream")]
    BuildInputStreamError(cpal::BuildStreamError),
    #[error("Failed to configure output stream")]
    BuildOutputStreamError(cpal::BuildStreamError),
    #[error("Failed to start input stream")]
    InputStreamError(cpal::PlayStreamError),
    #[error("Failed to start output stream")]
    OutputStreamError(cpal::PlayStreamError),
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
        let input_stream = build_input_stream(input_tuple, producer)?;
        let output_stream = build_output_stream(
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

fn build_output_stream(
    mut app: impl StandaloneProcessor,
    mut midi_context: Option<MidiContext>,
    num_output_channels: usize,
    num_input_channels: usize,
    mut input_consumer: Consumer<f32>,
    output_device: Device,
    output_config: StreamConfig,
) -> Result<Stream, AudioThreadError> {
    // Output callback section
    log::info!(
        "num_input_channels={} num_output_channels={} sample_rate={}",
        num_input_channels,
        num_output_channels,
        output_config.sample_rate.0
    );
    let output_stream = output_device
        .build_output_stream(
            &output_config,
            move |data: &mut [f32], _output_info: &cpal::OutputCallbackInfo| {
                output_stream_with_context(
                    midi_context.as_mut(),
                    &mut app,
                    num_input_channels,
                    num_output_channels,
                    &mut input_consumer,
                    data,
                );
            },
            |err| {
                log::error!("Playback error: {:?}", err);
            },
        )
        .map_err(AudioThreadError::BuildOutputStreamError)?;

    Ok(output_stream)
}

fn build_input_stream(
    input_tuple: Option<(Device, StreamConfig)>,
    mut producer: Producer<f32>,
) -> Result<Option<Stream>, AudioThreadError> {
    let input_stream = input_tuple.as_ref().map(|(input_device, input_config)| {
        input_device
            .build_input_stream(
                input_config,
                move |data: &[f32], _input_info: &cpal::InputCallbackInfo| {
                    input_stream_callback(&mut producer, data)
                },
                |err| {
                    log::error!("Input error: {:?}", err);
                },
            )
            .map_err(AudioThreadError::BuildInputStreamError)
    });

    let input_stream = if let Some(input_stream) = input_stream {
        Some(input_stream?)
    } else {
        None
    };

    Ok(input_stream)
}

fn configure_input_device(
    host: &Host,
    options: &StandaloneOptions,
    buffer_size: usize,
    sample_rate: usize,
) -> (Device, StreamConfig) {
    let input_device = options
        .input_device
        .as_ref()
        .map(|device_name| {
            let mut input_devices = host.input_devices().unwrap();
            input_devices.find(|device| matches!(device.name(), Ok(name) if &name == device_name))
        })
        .flatten()
        .unwrap_or_else(|| host.default_input_device().unwrap());
    let supported_configs = input_device.supported_input_configs().unwrap();
    let mut supports_stereo = false;
    for config in supported_configs {
        log::info!("  INPUT Supported config: {:?}", config);
        if config.channels() > 1 {
            supports_stereo = true;
        }
    }

    let input_config = input_device.default_input_config().unwrap();
    let mut input_config: StreamConfig = input_config.into();
    input_config.channels = if supports_stereo { 2 } else { 1 };
    input_config.sample_rate = SampleRate(sample_rate as u32);
    input_config.buffer_size = BufferSize::Fixed(buffer_size as u32);

    #[cfg(target_os = "ios")]
    {
        input_config.buffer_size = BufferSize::Default;
    }

    log::info!(
        "Using input name={} sample_rate={} buffer_size={:?}",
        input_device.name().unwrap(),
        sample_rate,
        input_config.buffer_size
    );

    (input_device, input_config)
}

fn configure_output_device(
    host: Host,
    options: &StandaloneOptions,
    buffer_size: usize,
    sample_rate: usize,
) -> (Device, StreamConfig) {
    let output_device = options
        .input_device
        .as_ref()
        .map(|device_name| {
            let mut output_devices = host.output_devices().unwrap();
            output_devices.find(|device| matches!(device.name(), Ok(name) if &name == device_name))
        })
        .flatten()
        .unwrap_or_else(|| host.default_output_device().unwrap());
    for config in output_device.supported_output_configs().unwrap() {
        log::info!("  OUTPUT Supported config: {:?}", config);
    }
    let output_config = output_device.default_output_config().unwrap();
    let mut output_config: StreamConfig = output_config.into();
    output_config.channels = output_device
        .supported_output_configs()
        .unwrap()
        .map(|config| config.channels())
        .max()
        .unwrap_or(2)
        .min(2);
    output_config.sample_rate = SampleRate(sample_rate as u32);
    output_config.buffer_size = BufferSize::Fixed(buffer_size as u32);

    #[cfg(target_os = "ios")]
    {
        output_config.buffer_size = BufferSize::Default;
    }

    log::info!(
        "Using output name={} sample_rate={} buffer_size={:?}",
        output_device.name().unwrap(),
        sample_rate,
        output_config.buffer_size
    );
    (output_device, output_config)
}

fn input_stream_callback(producer: &mut Producer<f32>, data: &[f32]) {
    for sample in data {
        while producer.push(*sample).is_err() {}
    }
}

fn output_stream_with_context<Processor: StandaloneProcessor>(
    midi_context: Option<&mut MidiContext>,
    processor: &mut Processor,
    // 1-2
    num_input_channels: usize,
    // 1-2
    num_output_channels: usize,
    consumer: &mut Consumer<f32>,
    data: &mut [f32],
) {
    let mut audio_buffer = InterleavedAudioBuffer::new(num_output_channels, data);

    for frame in audio_buffer.frames_mut() {
        if num_input_channels == num_output_channels {
            for sample in frame {
                if let Some(input_sample) = consumer.pop() {
                    *sample = input_sample;
                } else {
                }
            }
        } else if let Some(input_sample) = consumer.pop() {
            for sample in frame {
                *sample = input_sample
            }
        } else {
            break;
        }
    }

    // Collect MIDI
    flush_midi_events(midi_context, processor);

    processor.processor().process(&mut audio_buffer);
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
