use audio_processor_standalone::audio_processor_main;
use metronome::MetronomeProcessor;

fn main() {
    let metronome = MetronomeProcessor::default();
    audio_processor_main(metronome);
}
