use std::sync::atomic::Ordering;
use std::sync::Mutex;
use std::time::Duration;

use anyhow::Result;
use flutter_rust_bridge::StreamSink;
use lazy_static::lazy_static;

use audio_garbage_collector::Shared;
use audio_processor_standalone::standalone_processor::StandaloneOptions;
use audio_processor_standalone::{standalone_start, StandaloneAudioOnlyProcessor};

use crate::MetronomeProcessor;
use crate::MetronomeProcessorHandle;

mod state;
use state::{with_state, with_state0, State, STATE};

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
        state
            .processor_handle
            .is_playing
            .store(value, Ordering::Relaxed);
    })
}

pub fn set_tempo(value: f32) -> Result<i32> {
    with_state0(|state| {
        state.processor_handle.tempo.set(value);
    })
}

pub fn set_volume(value: f32) -> Result<i32> {
    with_state0(|state| {
        state.processor_handle.volume.set(value);
    })
}

pub fn set_beats_per_bar(value: i32) -> Result<i32> {
    with_state0(|state| {
        state
            .processor_handle
            .beats_per_bar
            .store(value, Ordering::Relaxed);
    })
}

pub fn get_playhead(sink: StreamSink<f32>) -> Result<i32> {
    with_state(|state| {
        let handle = state.processor_handle.clone();
        std::thread::spawn(move || {
            loop {
                sink.add(handle.position_beats.get());
                std::thread::sleep(Duration::from_millis(50));
            }
            // sink.close();
        });
        Ok(0)
    })
}
