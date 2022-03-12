use audio_processor_traits::BufferProcessor;
use audio_processor_utility::gain::GainProcessor;

fn main() {
    let gain = GainProcessor::default();
    gain.set_gain(0.5);
    audio_processor_standalone::audio_processor_main(BufferProcessor(gain));
}
