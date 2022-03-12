use audio_processor_time::MonoDelayProcessor;
use audio_processor_traits::BufferProcessor;

fn main() {
    let delay = BufferProcessor(MonoDelayProcessor::default());
    audio_processor_standalone::audio_processor_main(delay);
}
