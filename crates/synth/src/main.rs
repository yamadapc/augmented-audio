use audio_garbage_collector::GarbageCollector;
use audio_processor_standalone::audio_processor_main_with_midi;
use synth::Synthesizer;

fn main() {
    wisual_logger::init_from_env();
    let garbage_collector = GarbageCollector::default();
    let processor = Synthesizer::new(44100.0);
    audio_processor_main_with_midi(processor, garbage_collector.handle());
}
