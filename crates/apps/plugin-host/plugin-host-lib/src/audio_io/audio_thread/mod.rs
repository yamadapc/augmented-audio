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
use basedrop::{Handle, Shared, SharedCell};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamConfig;
use ringbuf::Consumer;

use audio_processor_graph::AudioProcessorGraph;
use audio_processor_standalone_midi::audio_thread::MidiAudioThreadHandler;
use audio_processor_standalone_midi::host::MidiMessageQueue;

use audio_processor_traits::{AudioBuffer, AudioContext};
use audio_processor_traits::{AudioProcessor, AudioProcessorSettings, SilenceAudioProcessor};
use error::AudioThreadError;
use options::AudioThreadOptions;

use crate::audio_io::audio_thread::options::{AudioDeviceId, AudioHostId};
use crate::processors::shared_processor::{ProcessorCell, SharedProcessor};
use crate::processors::test_host_processor::TestHostProcessor;

mod cpal_option_handling;
pub mod error;
pub mod options;

#[allow(clippy::large_enum_variant)]
pub enum AudioThreadProcessor {
    Active(TestHostProcessor),
    Graph(AudioProcessorGraph),
    Silence(SilenceAudioProcessor<f32>),
}

/// Centralizes work done around the CPAL audio thread.
///
/// Holds an atomic reference to the current processor, which may be hot-swapped while the audio
/// thread is running.
pub struct AudioThread {
    processor: Shared<SharedCell<ProcessorCell<AudioThreadProcessor>>>,
    processor_ref: SharedProcessor<AudioThreadProcessor>,
    output_stream: Option<cpal::Stream>,
    input_stream: Option<cpal::Stream>,
    audio_thread_options: AudioThreadOptions,
    midi_message_queue: MidiMessageQueue,
}

unsafe impl Send for AudioThread {}

impl AudioThread {
    fn new(
        handle: &Handle,
        midi_message_queue: MidiMessageQueue,
        audio_thread_options: AudioThreadOptions,
    ) -> Self {
        let processor = AudioThreadProcessor::Silence(SilenceAudioProcessor::new());
        let processor_ref = SharedProcessor::new(handle, processor);
        AudioThread {
            processor: Shared::new(handle, SharedCell::new(processor_ref.shared())),
            processor_ref,
            output_stream: None,
            input_stream: None,
            midi_message_queue,
            audio_thread_options,
        }
    }

    pub fn start_audio(&mut self) -> Result<(), AudioThreadError> {
        let processor = self.processor.clone();
        let audio_thread_options = self.audio_thread_options.clone();
        let midi_message_queue = self.midi_message_queue.clone();
        let (maybe_input_stream, output_stream) =
            create_stream(&audio_thread_options, processor, midi_message_queue)?;
        log::info!(
            "Starting CPAL output stream options={:?}",
            audio_thread_options
        );
        if let Some(input_stream) = maybe_input_stream.as_ref() {
            input_stream.play()?;
        }
        output_stream.play()?;
        self.output_stream = Some(output_stream);
        self.input_stream = maybe_input_stream;
        log::info!("CPAL streams started");
        Ok(())
    }

    /// Change audio host & restart audio thread
    pub fn set_host_id(&mut self, host_id: AudioHostId) -> Result<(), AudioThreadError> {
        if host_id != self.audio_thread_options.host_id {
            self.audio_thread_options.host_id = host_id;
            self.wait()?;
            self.start_audio()?;
        }
        Ok(())
    }

    /// Change input device & restart audio thread
    pub fn set_input_device_id(
        &mut self,
        input_device_id: Option<AudioDeviceId>,
    ) -> Result<(), AudioThreadError> {
        // if input_device_id != self.audio_thread_options.input_device_id {
        self.audio_thread_options.input_device_id = input_device_id;
        self.wait()?;
        self.start_audio()?;
        // }
        Ok(())
    }

    /// Change output device & restart audio thread
    pub fn set_output_device_id(
        &mut self,
        output_device_id: AudioDeviceId,
    ) -> Result<(), AudioThreadError> {
        // if output_device_id != self.audio_thread_options.output_device_id {
        self.audio_thread_options.output_device_id = output_device_id;
        self.wait()?;
        self.start_audio()?;
        // }
        Ok(())
    }

    pub fn wait(&self) -> Result<(), AudioThreadError> {
        if let Some(stream) = &self.input_stream {
            log::info!("Pausing the input stream");
            stream.pause()?;
        }
        if let Some(stream) = &self.output_stream {
            log::info!("Pausing the output stream");
            stream.pause()?;
        }
        Ok(())
    }

    /// # Safety:
    /// The processor MUST be prepared for playback when it's set.
    pub fn set_processor(&mut self, processor: SharedProcessor<AudioThreadProcessor>) {
        self.processor_ref = processor.clone();
        let _old_processor_ptr = self.processor.replace(processor.shared());
        // Let the old processor be dropped
    }

    pub fn default_settings() -> Result<AudioProcessorSettings, AudioThreadError> {
        let cpal_host = cpal::default_host();
        let output_device = cpal_host
            .default_output_device()
            .ok_or(AudioThreadError::OutputDeviceNotFoundError)?;
        let output_config = output_device.default_output_config()?;
        let output_config: StreamConfig = output_config.into();
        let channels = output_config.channels as usize;
        let audio_settings = AudioProcessorSettings::new(
            AudioThreadOptions::default().sample_rate.0 as f32,
            channels,
            channels,
            512,
        );

        Ok(audio_settings)
    }
}

fn create_stream(
    options: &AudioThreadOptions,
    processor: Shared<SharedCell<ProcessorCell<AudioThreadProcessor>>>,
    midi_message_queue: MidiMessageQueue,
) -> Result<(Option<cpal::Stream>, cpal::Stream), AudioThreadError> {
    let host = cpal_option_handling::get_cpal_host(&options.host_id);

    let output_device =
        cpal_option_handling::get_cpal_output_device(&host, &options.output_device_id)?;
    let output_config = cpal_option_handling::get_output_config(options, &output_device)?;
    let input_device = if let Some(device_id) = &options.input_device_id {
        Some(cpal_option_handling::get_cpal_input_device(
            &host, device_id,
        )?)
    } else {
        None
    };
    let input_config = if let Some(device) = &input_device {
        Some(cpal_option_handling::get_input_config(options, device)?)
    } else {
        None
    };

    log::info!(
        "Creating streams\n  output_device={} output_sample_rate={} input_device={:?} input_sample_rate={}",
        output_device.name()?,
        output_config.sample_rate.0,
        options.input_device_id,
        input_config.as_ref().map(|c| c.sample_rate.0).unwrap_or(0),
    );
    let stream = create_stream_inner(
        processor,
        &output_device,
        &output_config,
        input_device.as_ref().zip(input_config.as_ref()),
        midi_message_queue,
    )?;
    Ok(stream)
}

fn create_stream_inner(
    processor: Shared<SharedCell<ProcessorCell<AudioThreadProcessor>>>,
    output_device: &cpal::Device,
    output_config: &cpal::StreamConfig,
    input: Option<(&cpal::Device, &cpal::StreamConfig)>,
    midi_message_queue: MidiMessageQueue,
) -> Result<(Option<cpal::Stream>, cpal::Stream), AudioThreadError> {
    let buffer_size = match output_config.buffer_size {
        cpal::BufferSize::Default => Err(AudioThreadError::UnexpectedDefaultBufferSize),
        cpal::BufferSize::Fixed(buffer_size) => Ok(buffer_size),
    }?;

    log::info!("Buffer size {:?}", buffer_size);

    let num_channels: usize = output_config.channels.into();
    let mut midi_message_handler = MidiAudioThreadHandler::default();

    let buffer = ringbuf::RingBuffer::new((buffer_size * 4) as usize);
    let (mut producer, mut consumer) = buffer.split();
    let error_callback = |err| {
        log::error!("Playback error: {:?}", err);
    };

    let input_stream = if let Some((input_device, input_config)) = input {
        log::info!("Initializing input device {}", input_device.name().unwrap());
        let input_stream = input_device.build_input_stream(
            input_config,
            move |data: &[f32], _input_info: &cpal::InputCallbackInfo| {
                input_stream_callback(&mut producer, data)
            },
            error_callback,
            None,
        )?;
        Some(input_stream)
    } else {
        None
    };

    let mut audio_buffer = AudioBuffer::empty();
    audio_buffer.resize(num_channels, buffer_size as usize);
    let has_input = input_stream.is_some();
    log::info!("OUTPUT has_input={}", has_input);
    let output_stream = output_device.build_output_stream(
        output_config,
        move |data: &mut [f32], _output_info: &cpal::OutputCallbackInfo| {
            output_stream_callback(
                &processor,
                &midi_message_queue,
                &mut midi_message_handler,
                &mut consumer,
                has_input,
                &mut audio_buffer,
                data,
            );
        },
        error_callback,
        None,
    )?;

    Ok((input_stream, output_stream))
}

fn output_stream_callback(
    processor: &Shared<SharedCell<ProcessorCell<AudioThreadProcessor>>>,
    midi_message_queue: &MidiMessageQueue,
    midi_message_handler: &mut MidiAudioThreadHandler,
    consumer: &mut Consumer<f32>,
    has_input: bool,
    audio_buffer: &mut AudioBuffer<f32>,
    data: &mut [f32],
) {
    if has_input {
        for sample in data.iter_mut() {
            if let Some(input_sample) = consumer.pop() {
                *sample = input_sample;
            }
        }
    }

    midi_message_handler.collect_midi_messages(midi_message_queue);

    audio_buffer.copy_from_interleaved(data);

    if !has_input {
        for sample in audio_buffer.slice_mut() {
            *sample = 0.0;
        }
    }

    let shared_processor = processor.get();
    let processor_ptr = shared_processor.0.get();
    let mut context = AudioContext::default(); // TODO
    match unsafe { &mut (*processor_ptr) } {
        AudioThreadProcessor::Active(processor) => {
            processor.process_midi(midi_message_handler.buffer());
            processor.process(&mut context, audio_buffer)
        }
        AudioThreadProcessor::Silence(processor) => {
            (*processor).process(&mut context, audio_buffer)
        }
        AudioThreadProcessor::Graph(graph) => {
            // graph.process_midi(midi_message_handler.buffer());
            graph.process(&mut context, audio_buffer);
        }
    }

    midi_message_handler.clear();
}

fn input_stream_callback(producer: &mut ringbuf::Producer<f32>, data: &[f32]) {
    for sample in data {
        while producer.push(*sample).is_err() {}
    }
}

pub mod actor {
    use actix::{
        Actor, AsyncContext, Context, Handler, Message, Supervised, SystemService, WrapFuture,
    };

    use audio_processor_standalone_midi::host::{GetQueueMessage, MidiHost};

    use super::*;

    impl Actor for AudioThread {
        type Context = Context<Self>;
    }

    impl Supervised for AudioThread {}

    impl Default for AudioThread {
        fn default() -> Self {
            AudioThread::new(
                audio_garbage_collector::handle(),
                Shared::new(
                    audio_garbage_collector::handle(),
                    atomic_queue::Queue::new(0),
                ),
                Default::default(),
            )
        }
    }

    impl SystemService for AudioThread {
        fn service_started(&mut self, ctx: &mut Context<Self>) {
            log::info!("AudioThread started");
            let midi_host = MidiHost::from_registry();
            let own_addr = ctx.address();
            ctx.spawn(
                async move {
                    let queue = midi_host.send(GetQueueMessage).await.unwrap();
                    if let Err(err) = own_addr
                        .send(AudioThreadMessage::SetQueue(queue.0))
                        .await
                        .unwrap()
                    {
                        log::error!("Failed to set AudioThread MIDI queue {}", err);
                    }
                }
                .into_actor(self),
            );
        }
    }

    #[derive(Message)]
    #[rtype(result = "Result<(), AudioThreadError>")]
    pub enum AudioThreadMessage {
        Start,
        SetQueue(MidiMessageQueue),
        SetOptions {
            host_id: AudioHostId,
            input_device_id: Option<AudioDeviceId>,
            output_device_id: AudioDeviceId,
        },
        SetHost {
            host_id: AudioHostId,
        },
        SetInputDevice {
            input_device_id: Option<AudioDeviceId>,
        },
        SetOutputDevice {
            output_device_id: AudioDeviceId,
        },
        SetProcessor {
            processor: SharedProcessor<AudioThreadProcessor>,
        },
        Wait,
    }

    impl Handler<AudioThreadMessage> for AudioThread {
        type Result = Result<(), AudioThreadError>;

        fn handle(&mut self, msg: AudioThreadMessage, _ctx: &mut Self::Context) -> Self::Result {
            use AudioThreadMessage::*;

            match msg {
                Start => self.start_audio(),
                SetQueue(queue) => {
                    self.midi_message_queue = queue;
                    if self.output_stream.is_some() {
                        self.wait()?;
                        self.start_audio()?;
                    }
                    Ok(())
                }
                SetOptions {
                    host_id,
                    input_device_id,
                    output_device_id,
                } => {
                    let mut audio_thread_options = self.audio_thread_options.clone();
                    audio_thread_options.host_id = host_id;
                    audio_thread_options.input_device_id = input_device_id;
                    audio_thread_options.output_device_id = output_device_id;
                    if audio_thread_options != self.audio_thread_options {
                        self.audio_thread_options = audio_thread_options;
                        self.wait()?;
                        self.start_audio()?;
                    }
                    Ok(())
                }
                SetHost { host_id } => self.set_host_id(host_id),
                SetInputDevice { input_device_id } => self.set_input_device_id(input_device_id),
                SetOutputDevice { output_device_id } => self.set_output_device_id(output_device_id),
                SetProcessor { processor } => {
                    self.set_processor(processor);
                    Ok(())
                }
                Wait => self.wait(),
            }
        }
    }

    #[cfg(target_os = "macos")]
    #[cfg(test)]
    mod test {
        use atomic_queue::Queue;
        use audio_garbage_collector::GarbageCollector;

        use super::*;

        #[actix::test]
        async fn test_start_audio_thread() {
            let _ = wisual_logger::try_init_from_env();

            let gc = GarbageCollector::default();
            let midi_queue = Shared::new(gc.handle(), Queue::new(100));
            let audio_thread =
                AudioThread::new(gc.handle(), midi_queue, Default::default()).start();

            audio_thread
                .send(AudioThreadMessage::Start)
                .await
                .unwrap()
                .unwrap();
            audio_thread
                .send(AudioThreadMessage::Wait)
                .await
                .unwrap()
                .unwrap();
        }
    }
}
