use audio_processor_time::FreeverbProcessor;
use audio_processor_traits::BufferProcessor;

fn main() {
    let delay = BufferProcessor(FreeverbProcessor::default());
    audio_processor_standalone::audio_processor_main(delay);
}
