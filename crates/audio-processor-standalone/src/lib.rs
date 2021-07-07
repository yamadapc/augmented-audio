use basedrop::Handle;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, Host, SampleRate, StreamConfig};
use ringbuf::{Consumer, Producer};

use audio_processor_standalone_midi::audio_thread::MidiAudioThreadHandler;
use audio_processor_standalone_midi::host::{MidiHost, MidiMessageQueue};
use audio_processor_traits::{
    AudioProcessor, AudioProcessorSettings, InterleavedAudioBuffer, MidiEventHandler,
};

// TODO Fix duplication in this file due to MIDI vs. no MIDI.
pub fn audio_processor_main_with_midi<
    Processor: AudioProcessor<SampleType = f32> + MidiEventHandler + 'static,
>(
    mut audio_processor: Processor,
    handle: &Handle,
) {
    let _ = wisual_logger::try_init_from_env();

    // MIDI set-up
    let mut midi_host = MidiHost::default_with_handle(handle);
    midi_host.start().expect("Failed to start MIDI host");
    let midi_message_queue = midi_host.messages().clone();
    let mut midi_audio_thread_handler = MidiAudioThreadHandler::default();

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
    audio_processor.prepare(settings);

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
                output_stream_callback_with_midi(
                    &midi_message_queue,
                    &mut midi_audio_thread_handler,
                    &mut audio_processor,
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

    std::thread::park();
}

pub fn audio_processor_main<Processor: AudioProcessor<SampleType = f32> + 'static>(
    mut audio_processor: Processor,
) {
    let _ = wisual_logger::try_init_from_env();

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
    audio_processor.prepare(settings);

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

    let output_stream = output_device
        .build_output_stream(
            &output_config,
            move |data: &mut [f32], _output_info: &cpal::OutputCallbackInfo| {
                output_stream_callback(&mut audio_processor, num_channels, &mut consumer, data);
            },
            |err| {
                log::error!("Playback error: {:?}", err);
            },
        )
        .unwrap();

    output_stream.play().unwrap();
    input_stream.play().unwrap();

    std::thread::park();
}

fn configure_input_device(
    host: &Host,
    buffer_size: u32,
    sample_rate: u32,
) -> (cpal::Device, StreamConfig) {
    let input_device = host.default_input_device().unwrap();
    let input_config = input_device.default_input_config().unwrap();
    let mut input_config: StreamConfig = input_config.into();
    log::info!("Using input: {}", input_device.name().unwrap());
    input_config.sample_rate = SampleRate(sample_rate);
    input_config.buffer_size = BufferSize::Fixed(buffer_size);
    (input_device, input_config)
}

fn configure_output_device(
    host: Host,
    buffer_size: u32,
    sample_rate: u32,
) -> (cpal::Device, usize, StreamConfig) {
    let output_device = host.default_output_device().unwrap();
    let output_config = output_device.default_output_config().unwrap();
    let num_channels: usize = output_config.channels().into();
    let mut output_config: StreamConfig = output_config.into();
    log::info!("Using output: {}", output_device.name().unwrap());
    output_config.sample_rate = SampleRate(sample_rate);
    output_config.buffer_size = BufferSize::Fixed(buffer_size);
    (output_device, num_channels, output_config)
}

fn input_stream_callback(producer: &mut Producer<f32>, data: &[f32]) {
    let mut output_behind = false;
    for sample in data {
        while producer.push(*sample).is_err() {
            output_behind = true;
        }
    }
    if output_behind {
        log::error!("Output is behind");
    }
}

fn output_stream_callback<Processor: AudioProcessor<SampleType = f32> + 'static>(
    audio_processor: &mut Processor,
    num_channels: usize,
    consumer: &mut Consumer<f32>,
    data: &mut [f32],
) {
    let mut input_behind = false;

    for sample in data.iter_mut() {
        if let Some(input_sample) = consumer.pop() {
            *sample = input_sample;
        } else {
            input_behind = true;
        }
    }

    if input_behind {
        log::error!("Input is behind");
    }

    let mut audio_buffer = InterleavedAudioBuffer::new(num_channels, data);
    audio_processor.process(&mut audio_buffer);
}

fn output_stream_callback_with_midi<
    Processor: AudioProcessor<SampleType = f32> + MidiEventHandler + 'static,
>(
    midi_message_queue: &MidiMessageQueue,
    midi_audio_thread_handler: &mut MidiAudioThreadHandler,
    audio_processor: &mut Processor,
    num_channels: usize,
    consumer: &mut Consumer<f32>,
    data: &mut [f32],
) {
    let mut input_behind = false;

    for sample in data.iter_mut() {
        if let Some(input_sample) = consumer.pop() {
            *sample = input_sample;
        } else {
            input_behind = true;
        }
    }

    if input_behind {
        log::error!("Input is behind");
    }

    // Collect MIDI
    midi_audio_thread_handler.collect_midi_messages(midi_message_queue);
    audio_processor.process_midi_events(&midi_audio_thread_handler.buffer());
    midi_audio_thread_handler.clear();

    let mut audio_buffer = InterleavedAudioBuffer::new(num_channels, data);
    audio_processor.process(&mut audio_buffer);
}

#[cfg(test)]
mod test {
    #[test]
    fn test_compile() {}
}
