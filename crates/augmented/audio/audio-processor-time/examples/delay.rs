use audio_processor_standalone::generic_standalone_run;
use audio_processor_time::MonoDelayProcessor;
use audio_processor_traits::BufferProcessor;

fn main() {
    let delay = BufferProcessor(MonoDelayProcessor::default());
    generic_standalone_run!(delay);
}
