use std::sync::atomic::AtomicBool;

use audio_garbage_collector::make_shared;

use crate::audio::midi_map::MidiControllerNumber;
pub use crate::audio::multi_track_looper::midi_store::MidiEvent;
use crate::audio::multi_track_looper::midi_store::MidiStoreActor;
use crate::audio::multi_track_looper::parameters::EntityId;
use crate::ForeignCallback;
use crate::LooperEngine;

#[no_mangle]
pub unsafe extern "C" fn looper_engine__add_midi_mapping(
    engine: *mut LooperEngine,
    controller_number: i32,
    entity_id: EntityId,
) {
    log::info!(
        "Adding MIDI mapping controller_number={} parameter_id={:?}",
        controller_number,
        entity_id
    );
    let engine = &(*engine);

    let midi_store = &engine.midi_store;
    midi_store.midi_map().add(
        MidiControllerNumber::new(controller_number as u8),
        entity_id,
    );
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__register_midi_callback(
    engine: *mut LooperEngine,
    callback: ForeignCallback<MidiEvent>,
) {
    let engine = &(*engine);

    let callback = Box::new(move |inner| {
        callback.call(inner);
    });

    let midi_store = &engine.midi_store;
    let midi_store = midi_store;
    let midi_actor_is_running = make_shared(AtomicBool::new(true));
    let mut midi_actor =
        MidiStoreActor::new(midi_store.queue().clone(), midi_actor_is_running, callback);

    std::thread::Builder::new()
        .name(String::from("midi-forwarding-thread"))
        .spawn(move || {
            midi_actor.run();
        })
        .unwrap();
}
