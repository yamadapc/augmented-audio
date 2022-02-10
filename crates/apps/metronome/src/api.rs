//! This module contains the public API exposed to flutter
use std::sync::atomic::Ordering;

use std::time::Duration;

use anyhow::Result;
use flutter_rust_bridge::StreamSink;

mod state;
use state::{with_state, with_state0};

pub fn initialize() -> Result<i32> {
    state::initialize();
    Ok(0)
}

pub fn deinitialize() -> Result<i32> {
    state::deinitialize();
    Ok(0)
}

pub fn set_is_playing(value: bool) -> Result<i32> {
    with_state0(|state| {
        state.processor_handle.set_is_playing(value);
    })
}

pub fn set_tempo(value: f32) -> Result<i32> {
    with_state0(|state| {
        state.processor_handle.set_tempo(value);
    })
}

pub fn set_volume(value: f32) -> Result<i32> {
    with_state0(|state| state.processor_handle.set_volume(value))
}

pub fn set_beats_per_bar(value: i32) -> Result<i32> {
    with_state0(|state| {
        state.processor_handle.set_beats_per_bar(value);
    })
}

pub fn get_playhead(sink: StreamSink<f32>) -> Result<i32> {
    with_state(|state| {
        let handle = state.processor_handle.clone();
        std::thread::spawn(move || {
            loop {
                sink.add(handle.position_beats());
                std::thread::sleep(Duration::from_millis(50));
            }
            // sink.close();
        });
        Ok(0)
    })
}
