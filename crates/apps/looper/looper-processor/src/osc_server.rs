use basedrop::Shared;

use audio_processor_standalone_osc::{OscMap, OscServer};

use crate::{LooperId, MultiTrackLooperHandle};

pub fn setup_osc_server(handle: Shared<MultiTrackLooperHandle>) {
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
        if let Err(err) = osc_server.start() {
            log::error!("OscServer has exited with {}", err);
        }
    });
}
