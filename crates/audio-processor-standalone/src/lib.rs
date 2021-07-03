use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use audio_processor_traits::{AudioProcessor, InterleavedAudioBuffer};

pub fn audio_processor_main<Processor: AudioProcessor<SampleType = f32> + 'static>(
    mut audio_processor: Processor,
) {
    let host = cpal::default_host();
    let output_device = host.default_output_device().unwrap();
    let output_config = output_device.default_output_config().unwrap();
    let num_channels: usize = output_config.channels().into();
    let output_config = output_config.into();

    let stream = output_device
        .build_output_stream(
            &output_config,
            move |data: &mut [f32], _output_info: &cpal::OutputCallbackInfo| {
                let mut audio_buffer = InterleavedAudioBuffer::new(num_channels, data);
                audio_processor.process(&mut audio_buffer);
            },
            |err| {
                log::error!("Playback error: {:?}", err);
            },
        )
        .unwrap();
    stream.play().unwrap();
    std::thread::park();
}

#[cfg(test)]
mod test {
    #[test]
    fn test_compile() {}
}
