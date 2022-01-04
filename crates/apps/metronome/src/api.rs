use std::sync::atomic::Ordering;
use std::sync::Mutex;

use anyhow::Result;
use audio_garbage_collector::Shared;
use audio_processor_standalone::audio_processor_start;
use lazy_static::lazy_static;

use crate::MetronomeProcessor;
use crate::MetronomeProcessorHandle;

struct State {
    _handles: audio_processor_standalone::StandaloneHandles,
    processor_handle: Shared<MetronomeProcessorHandle>,
}

unsafe impl Send for State {}

impl State {
    fn new() -> Self {
        let processor = MetronomeProcessor::new();
        let processor_handle = processor.handle.clone();
        let handles = audio_processor_start(processor);
        Self {
            processor_handle,
            _handles: handles,
        }
    }
}

lazy_static! {
    static ref STATE: Mutex<Option<State>> = Mutex::new(None);
}

pub fn initialize() -> Result<i32> {
    let mut handles = STATE.lock().unwrap();
    *handles = Some(State::new());
    Ok(0)
}

pub fn deinitialize() -> Result<i32> {
    let mut handles = STATE.lock().unwrap();
    *handles = None;
    Ok(0)
}

fn with_state0(f: impl FnOnce(&State) -> ()) -> Result<i32> {
    with_state(|state| {
        f(state);
        Ok(0)
    })
}

fn with_state<T>(f: impl FnOnce(&State) -> Result<T>) -> Result<T> {
    let handles = STATE.lock().unwrap();
    if let Some(state) = &*handles {
        f(state)
    } else {
        Err(anyhow::Error::msg(
            "Failed to lock state. `initialize` needs to be called.",
        ))
    }
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

pub fn get_playhead() -> Result<f32> {
    with_state(|state| Ok(state.processor_handle.position_beats.get()))
}
