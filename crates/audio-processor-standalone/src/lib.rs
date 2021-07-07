use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, SampleRate, StreamConfig};
use ringbuf::{Consumer, Producer};

use audio_processor_traits::{AudioProcessor, AudioProcessorSettings, InterleavedAudioBuffer};
#[cfg(target_arch = "wasm32")]
use wasm::*;

pub fn audio_processor_main<Processor: AudioProcessor<SampleType = f32> + 'static>(
    mut audio_processor: Processor,
) {
    let _ = wisual_logger::try_init_from_env();

    let host = cpal::default_host();
    log::info!("Using host: {}", host.id().name());

    let input_device = host.default_input_device().unwrap();
    let input_config = input_device.default_input_config().unwrap();
    let mut input_config: StreamConfig = input_config.into();
    log::info!("Using input: {}", input_device.name().unwrap());

    let output_device = host.default_output_device().unwrap();
    let output_config = output_device.default_output_config().unwrap();
    let num_channels: usize = output_config.channels().into();
    let mut output_config: StreamConfig = output_config.into();
    log::info!("Using output: {}", output_device.name().unwrap());

    let buffer_size = 512;
    input_config.sample_rate = SampleRate(44100);
    output_config.sample_rate = SampleRate(44100);
    output_config.buffer_size = BufferSize::Fixed(buffer_size);
    input_config.buffer_size = BufferSize::Fixed(buffer_size);

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

#[cfg(test)]
mod test {
    #[test]
    fn test_compile() {}
}
