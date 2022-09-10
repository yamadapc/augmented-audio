use audio_processor_metronome::MetronomeProcessor;
use audio_processor_standalone::audio_processor_main;

fn main() {
    let metronome = MetronomeProcessor::default();
    metronome.handle().set_tempo(120.0);
    audio_processor_main(metronome);
}
