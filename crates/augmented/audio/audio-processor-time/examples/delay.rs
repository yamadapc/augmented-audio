use audio_processor_time::MonoDelayProcessor;

fn main() {
    let delay = MonoDelayProcessor::default();
    audio_processor_standalone::audio_processor_main(delay);
}
