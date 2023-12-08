// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
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
    engine: *const LooperEngine,
    controller_number: i32,
    entity_id: EntityId,
) {
    log::info!(
        "Adding MIDI mapping controller_number={} parameter_id={:?}",
        controller_number,
        entity_id
    );
    let engine = &(*engine);

    let midi_store = &engine.midi_store();
    midi_store.midi_map().add(
        MidiControllerNumber::new(controller_number as u8),
        entity_id,
    );
}

#[no_mangle]
pub unsafe extern "C" fn looper_engine__register_midi_callback(
    engine: *const LooperEngine,
    callback: ForeignCallback<MidiEvent>,
) {
    let engine = &(*engine);

    let callback = Box::new(move |inner| {
        callback.call(inner);
    });

    let midi_store = &engine.midi_store();
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
