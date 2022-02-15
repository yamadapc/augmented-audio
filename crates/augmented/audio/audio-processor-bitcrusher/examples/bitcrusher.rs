use audio_processor_bitcrusher::BitCrusherProcessor;
use audio_processor_traits::parameters::AudioProcessorHandleRef;

fn main() {
    let processor = BitCrusherProcessor::default();
    processor.handle().set_bit_rate(44100.0 / 4.0);

    let handle: AudioProcessorHandleRef = Box::new(processor.generic_handle());
    let _audio_handles = audio_processor_standalone::audio_processor_start(processor);

    audio_processor_standalone::gui::open(handle);
}
