use audio_processor_bitcrusher::BitCrusherProcessor;
use audio_processor_traits::parameters::AudioProcessorHandleRef;
use std::env::VarError;

fn main() {
    let processor = BitCrusherProcessor::default();
    processor.handle().set_bit_rate(44100.0 / 4.0);

    match std::env::var("GUI") {
        Ok(value) if value == "true" => {
            let handle: AudioProcessorHandleRef = Box::new(processor.generic_handle());
            let _audio_handles = audio_processor_standalone::audio_processor_start(processor);
            audio_processor_standalone::gui::open(handle);
        }
        _ => {
            audio_processor_standalone::audio_processor_main(processor);
        }
    }
}
