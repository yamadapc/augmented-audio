use audio_processor_bitcrusher::BitCrusherProcessor;

fn main() {
    let processor = BitCrusherProcessor::default();
    processor.handle().set_bit_rate(44100.0 / 4.0);
    let handle = Box::new(processor.handle().clone());
    audio_processor_standalone::audio_processor_gui(processor, handle);
}
