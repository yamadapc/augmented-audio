use std::sync::atomic::AtomicBool;

use audio_garbage_collector::make_shared;

pub use crate::multi_track_looper::midi_store::MidiEvent;
use crate::multi_track_looper::midi_store::MidiStoreActor;

use super::{ForeignCallback, LooperEngine};

#[no_mangle]
pub unsafe extern "C" fn looper_engine__register_midi_callback(
    engine: *mut LooperEngine,
    callback: ForeignCallback<MidiEvent>,
) {
    let callback = Box::new(move |value| callback.call(value));

    let midi_store = &(*engine).midi_store;
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
