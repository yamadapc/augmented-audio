use audio_garbage_collector::GarbageCollector;
use audio_processor_standalone::audio_processor_main;
use looper_processor::LooperProcessor;

fn main() {
    wisual_logger::init_from_env();
    let garbage_collector = GarbageCollector::default();
    let processor = LooperProcessor::new(garbage_collector.handle());
    audio_processor_main(processor);
}
