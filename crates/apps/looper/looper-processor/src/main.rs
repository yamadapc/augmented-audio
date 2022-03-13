use audio_garbage_collector::{GarbageCollector, Shared};
use audio_processor_standalone::audio_processor_main_with_midi;
use audio_processor_standalone_osc::{OscMap, OscServer};
use looper_processor::{setup_osc_server, LooperId, MultiTrackLooper, MultiTrackLooperHandle};

fn main() {
    wisual_logger::init_from_env();
    let garbage_collector = GarbageCollector::default();
    let processor = MultiTrackLooper::new(Default::default(), 8);
    setup_osc_server(processor.handle().clone());

    audio_processor_main_with_midi(processor, garbage_collector.handle());
}
