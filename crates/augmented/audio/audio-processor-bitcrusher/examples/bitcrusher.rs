use audio_processor_bitcrusher::BitCrusherProcessor;
use audio_processor_standalone::generic_standalone_run;

fn main() {
    let processor = BitCrusherProcessor::default();
    processor.handle().set_bit_rate(44100.0 / 4.0);
    generic_standalone_run!(processor);
}
