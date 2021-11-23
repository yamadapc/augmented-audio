use audio_processor_time::MonoDelayProcessor;

fn main() {
    let delay = MonoDelayProcessor::default();
    delay.handle().set_feedback(0.3);
    audio_processor_standalone::audio_processor_main(delay);
}
