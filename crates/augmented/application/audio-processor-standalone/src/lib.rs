use basedrop::Handle;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, Host, SampleRate, StreamConfig};
use ringbuf::{Consumer, Producer};

use audio_processor_standalone_midi::audio_thread::MidiAudioThreadHandler;
use audio_processor_standalone_midi::host::{MidiHost, MidiMessageQueue};
use audio_processor_traits::{
    AudioProcessor, AudioProcessorSettings, InterleavedAudioBuffer, MidiEventHandler,
    MidiMessageLike,
};

trait StandaloneProcessor: Send + 'static {
    type Processor: AudioProcessor<SampleType = f32>;
    type Midi: MidiEventHandler;

    fn processor(&mut self) -> &mut Self::Processor;
    fn midi(&mut self) -> Option<&mut Self::Midi> {
        None
    }
}

struct NoMidiEventHandler {}
impl MidiEventHandler for NoMidiEventHandler {
    fn process_midi_events<Message: MidiMessageLike>(&mut self, _midi_messages: &[Message]) {}
}

pub struct StandaloneAudioOnlyProcessor<P> {
    processor: P,
}

impl<P> StandaloneProcessor for StandaloneAudioOnlyProcessor<P>
where
    P: AudioProcessor<SampleType = f32> + Send + 'static,
{
    type Processor = P;
    type Midi = NoMidiEventHandler;

    fn processor(&mut self) -> &mut Self::Processor {
        &mut self.processor
    }

    fn midi(&mut self) -> Option<&mut Self::Midi> {
        None
    }
}

struct StandaloneProcessorImpl<P> {
    processor: P,
}

impl<P> StandaloneProcessor for StandaloneProcessorImpl<P>
where
    P: AudioProcessor<SampleType = f32> + Send + 'static,
    P: MidiEventHandler,
{
    type Processor = P;
    type Midi = P;

    fn processor(&mut self) -> &mut Self::Processor {
        &mut self.processor
    }

    fn midi(&mut self) -> Option<&mut Self::Midi> {
        Some(&mut self.processor)
    }
}

pub struct StandaloneHandles {
    pub input_stream: cpal::Stream,
    pub output_stream: cpal::Stream,
    pub midi_host: Option<MidiHost>,
}

fn standalone_start(
    mut app: impl StandaloneProcessor,
    handle: Option<&Handle>,
) -> StandaloneHandles {
    let _ = wisual_logger::try_init_from_env();

    let midi_host = app.midi().and(handle).map(|handle| {
        // MIDI set-up
        let mut midi_host = MidiHost::default_with_handle(handle);
        midi_host.start().expect("Failed to start MIDI host");
        midi_host
    });
    let mut midi_context = midi_host.as_ref().map(|midi_host| {
        let midi_message_queue = midi_host.messages().clone();
        let midi_audio_thread_handler = MidiAudioThreadHandler::default();
        MidiContext {
            midi_audio_thread_handler,
            midi_message_queue,
        }
    });

    // Audio set-up
    let host = cpal::default_host();
    log::info!("Using host: {}", host.id().name());
    let buffer_size = 512;
    let sample_rate = 44100;
    let (input_device, input_config) = configure_input_device(&host, buffer_size, sample_rate);
    let (output_device, num_channels, output_config) =
        configure_output_device(host, buffer_size, sample_rate);
    let settings = AudioProcessorSettings::new(
        output_config.sample_rate.0 as f32,
        input_config.channels.into(),
        output_config.channels.into(),
        buffer_size,
    );
    app.processor().prepare(settings);

    let buffer = ringbuf::RingBuffer::new((buffer_size * 10) as usize);
    let (mut producer, mut consumer) = buffer.split();
    let input_stream = input_device
        .build_input_stream(
            &input_config,
            move |data: &[f32], _input_info: &cpal::InputCallbackInfo| {
                input_stream_callback(&mut producer, data)
            },
            |err| {
                log::error!("Input error: {:?}", err);
            },
        )
        .unwrap();

    // Output callback section
    let output_stream = output_device
        .build_output_stream(
            &output_config,
            move |data: &mut [f32], _output_info: &cpal::OutputCallbackInfo| {
                output_stream_with_context(
                    midi_context.as_mut(),
                    &mut app,
                    num_channels,
                    &mut consumer,
                    data,
                );
            },
            |err| {
                log::error!("Playback error: {:?}", err);
            },
        )
        .unwrap();

    output_stream.play().unwrap();
    input_stream.play().unwrap();
    log::info!("Audio streams started");

    StandaloneHandles {
        input_stream,
        output_stream,
        midi_host,
    }
}

/// Run an [`AudioProcessor`] / [`MidiEventHandler`] as a stand-alone cpal app and forward MIDI
/// messages received on all inputs to it.
///
/// Will internally create [`cpal::Stream`], [`MidiHost`] and park the current thread. If the thread
/// is unparked the function will exit and the audio/MIDI threads will stop once these structures
/// are dropped.
pub fn audio_processor_main_with_midi<
    Processor: AudioProcessor<SampleType = f32> + MidiEventHandler + Send + 'static,
>(
    audio_processor: Processor,
    handle: &Handle,
) {
    let _handles = audio_processor_start_with_midi(audio_processor, handle);
    std::thread::park();
}

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
    let app = StandaloneProcessorImpl {
        processor: audio_processor,
    };
    standalone_start(app, Some(handle))
}

/// Run an [`AudioProcessor`] stand-alone cpal app.
///
/// Returns the [`cpal::Stream`] streams. The audio-thread will keep running until these are dropped.
pub fn audio_processor_main<Processor: AudioProcessor<SampleType = f32> + Send + 'static>(
    audio_processor: Processor,
) {
    let _handles = audio_processor_start(audio_processor);
    std::thread::park();
}

/// Start an [`AudioProcessor`] as a stand-alone cpal app>
///
/// Returns the [`cpal::Stream`] streams. The audio-thread will keep running until these are dropped.
pub fn audio_processor_start<Processor: AudioProcessor<SampleType = f32> + Send + 'static>(
    audio_processor: Processor,
) -> StandaloneHandles {
    let app = StandaloneAudioOnlyProcessor {
        processor: audio_processor,
    };
    standalone_start(app, None)
}

fn configure_input_device(
    host: &Host,
    buffer_size: usize,
    sample_rate: usize,
) -> (cpal::Device, StreamConfig) {
    let input_device = host.default_input_device().unwrap();
    log::info!("Using input: {}", input_device.name().unwrap());
    let supported_configs = input_device.supported_input_configs().unwrap();
    for config in supported_configs {
        log::info!("Supported config: {:?}", config);
    }
    let input_config = input_device.default_input_config().unwrap();
    let mut input_config: StreamConfig = input_config.into();
    input_config.channels = 2;
    input_config.sample_rate = SampleRate(sample_rate as u32);
    input_config.buffer_size = BufferSize::Fixed(buffer_size as u32);
    (input_device, input_config)
}

fn configure_output_device(
    host: Host,
    buffer_size: usize,
    sample_rate: usize,
) -> (cpal::Device, usize, StreamConfig) {
    let output_device = host.default_output_device().unwrap();
    log::info!("Using output: {}", output_device.name().unwrap());
    let supported_configs = output_device.supported_input_configs().unwrap();
    for config in supported_configs {
        log::info!("Supported config: {:?}", config);
    }
    let output_config = output_device.default_output_config().unwrap();
    let num_channels: usize = output_config.channels().into();
    let mut output_config: StreamConfig = output_config.into();
    output_config.channels = 2;
    output_config.sample_rate = SampleRate(sample_rate as u32);
    output_config.buffer_size = BufferSize::Fixed(buffer_size as u32);
    (output_device, num_channels, output_config)
}

fn input_stream_callback(producer: &mut Producer<f32>, data: &[f32]) {
    for sample in data {
        while producer.push(*sample).is_err() {}
    }
}

struct MidiContext {
    midi_message_queue: MidiMessageQueue,
    midi_audio_thread_handler: MidiAudioThreadHandler,
}

fn output_stream_with_context<Processor: StandaloneProcessor>(
    midi_context: Option<&mut MidiContext>,
    processor: &mut Processor,
    num_channels: usize,
    consumer: &mut Consumer<f32>,
    data: &mut [f32],
) {
    for sample in data.iter_mut() {
        if let Some(input_sample) = consumer.pop() {
            *sample = input_sample;
        }
    }

    // Collect MIDI
    if let Some(MidiContext {
        midi_audio_thread_handler,
        midi_message_queue,
    }) = midi_context
    {
        if let Some(midi_handler) = processor.midi() {
            midi_audio_thread_handler.collect_midi_messages(midi_message_queue);
            midi_handler.process_midi_events(midi_audio_thread_handler.buffer());
            midi_audio_thread_handler.clear();
        }
    }

    let mut audio_buffer = InterleavedAudioBuffer::new(num_channels, data);
    processor.processor().process(&mut audio_buffer);
}
