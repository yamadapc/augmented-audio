use audio_garbage_collector::{GarbageCollector, Shared};
use audio_processor_standalone::audio_processor_main_with_midi;
use audio_processor_standalone_osc::{OscMap, OscServer};
use looper_processor::{
    LooperId, MultiTrackLooper, MultiTrackLooperHandle,
};

fn main() {
    wisual_logger::init_from_env();
    let garbage_collector = GarbageCollector::default();
    let processor = MultiTrackLooper::new(Default::default(), 8);
    setup_osc_server(processor.handle().clone());

    audio_processor_main_with_midi(processor, garbage_collector.handle());
}

fn setup_osc_server(handle: Shared<MultiTrackLooperHandle>) {
    let mut osc_map: OscMap<Shared<MultiTrackLooperHandle>> = OscMap::default();
    osc_map.add(
        "/looper/record",
        Box::new(|handle, _msg| {
            log::info!("Toggle recording");
            handle.start_recording(LooperId(0))
        }),
    );

    osc_map.add(
        "/looper/play",
        Box::new(|handle, _msg| {
            log::info!("Toggle playback");
            handle.toggle_playback(LooperId(0))
        }),
    );

    osc_map.add(
        "/looper/clear",
        Box::new(|handle, _msg| {
            log::info!("Clear");
            handle.clear(LooperId(0));
        }),
    );

    let osc_server = OscServer::new(handle, osc_map);
    let _ = std::thread::spawn(move || {
        osc_server.start();
    });
}
