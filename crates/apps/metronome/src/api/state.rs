//! This module wraps a singleton instance of the standalone audio processor.
//!
//! This instance is held behind a mutex. The metronome handle itself wouldn't need locks, but is
//! currently using a lock here for simplicity. The AudioThread reads directly from its handle
//! without waiting on any locks.

use std::sync::Mutex;

use anyhow::Result;
use lazy_static::lazy_static;

use audio_garbage_collector::Shared;
use audio_processor_standalone::standalone_processor::StandaloneOptions;
use audio_processor_standalone::{standalone_start, StandaloneAudioOnlyProcessor};

use crate::processor::MetronomeProcessor;
use crate::processor::MetronomeProcessorHandle;

pub struct State {
    _handles: audio_processor_standalone::StandaloneHandles,
    pub processor_handle: Shared<MetronomeProcessorHandle>,
}

/// The `StandaloneHandles` aren't `Send`. The reason for this is that the `cpal::Stream` isn't
/// `Send`. It should be safe to share this value between threads as long as it can't accessed.
#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for State {}

impl State {
    pub fn new() -> Self {
        let processor = MetronomeProcessor::new();
        let processor_handle = processor.handle().clone();
        processor_handle.set_is_playing(false);
        let app = StandaloneAudioOnlyProcessor::new(
            processor,
            StandaloneOptions {
                accepts_input: false,
                ..Default::default()
            },
        );
        let handles = standalone_start(app, None);
        Self {
            processor_handle,
            _handles: handles,
        }
    }
}

lazy_static! {
    pub static ref STATE: Mutex<Option<State>> = Mutex::new(None);
}

pub fn initialize() {
    let mut state = STATE.lock().unwrap();
    *state = Some(State::new());
}

pub fn deinitialize() {
    let mut handles = STATE.lock().unwrap();
    *handles = None;
}

pub fn with_state0(f: impl FnOnce(&State)) -> Result<i32> {
    with_state(|state| {
        f(state);
        Ok(0)
    })
}

pub fn with_state<T>(f: impl FnOnce(&State) -> Result<T>) -> Result<T> {
    let state = STATE.lock().unwrap();
    if let Some(state) = &*state {
        f(state)
    } else {
        Err(anyhow::Error::msg(
            "Failed to lock state. `initialize` needs to be called.",
        ))
    }
}
