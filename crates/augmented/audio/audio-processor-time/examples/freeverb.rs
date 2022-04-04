use audio_processor_standalone::generic_standalone_run;
use audio_processor_time::FreeverbProcessor;
use audio_processor_traits::BufferProcessor;

fn main() {
    let reverb = BufferProcessor(FreeverbProcessor::default());
    generic_standalone_run!(reverb);
}
