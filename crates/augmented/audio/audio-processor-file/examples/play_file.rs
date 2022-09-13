use audio_processor_testing_helpers::relative_path;
use audio_processor_traits::{AudioProcessor, AudioProcessorSettings, InterleavedAudioBuffer};
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{BufferSize, SampleRate, StreamConfig};

fn main() {
    wisual_logger::init_from_env();
    let mut processor = audio_processor_file::AudioFileProcessor::from_path(
        audio_garbage_collector::handle(),
        Default::default(),
        &relative_path!("../../../../input-files/bass.wav"),
    )
    .unwrap();
    processor.prepare(Default::default());
    run_audio(processor);
}

fn run_audio(mut processor: impl AudioProcessor<SampleType = f32> + Send + 'static) {
    let host = cpal::default_host();
    let output_device = host.default_output_device().unwrap();
    let _handle = output_device
        .build_output_stream(
            &StreamConfig {
                buffer_size: BufferSize::Default,
                channels: 2,
                sample_rate: SampleRate(44100),
            },
            move |data, info| {
                let mut buffer = InterleavedAudioBuffer::new(2, data);
                processor.process(&mut buffer);
            },
            |err| {},
        )
        .unwrap();
    std::thread::park();
}
