use audio_processor_traits::BufferProcessor;
use audio_processor_utility::mono::StereoToMonoProcessor;

fn main() {
    let mono = StereoToMonoProcessor::default();
    audio_processor_standalone::audio_processor_main(BufferProcessor(mono));
}
