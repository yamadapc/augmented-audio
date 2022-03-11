use audio_garbage_collector::{GarbageCollector, Shared};
use audio_processor_standalone::audio_processor_main_with_midi;
use audio_processor_standalone_osc::{OscMap, OscServer};
use looper_processor::{LooperProcessor, LooperProcessorHandle};

fn main() {
    wisual_logger::init_from_env();
    let garbage_collector = GarbageCollector::default();
    let processor = LooperProcessor::from_options(Default::default());
    setup_osc_server(processor.handle().clone());

    audio_processor_main_with_midi(processor, garbage_collector.handle());
}

fn setup_osc_server(handle: Shared<LooperProcessorHandle>) {
    let mut osc_map: OscMap<Shared<LooperProcessorHandle>> = OscMap::default();
    osc_map.add(
        "/looper/record",
        Box::new(|handle, _msg| {
            log::info!("Toggle recording");
            handle.toggle_recording();
        }),
    );

    osc_map.add(
        "/looper/play",
        Box::new(|handle, _msg| {
            log::info!("Toggle playback");
            handle.toggle_playback();
        }),
    );

    osc_map.add(
        "/looper/clear",
        Box::new(|handle, _msg| {
            log::info!("Clear");
            handle.clear();
        }),
    );

    let osc_server = OscServer::new(handle, osc_map);
    let _ = std::thread::spawn(move || {
        osc_server.start();
    });
}
