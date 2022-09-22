// = copyright ====================================================================
// Simple Metronome: macOS Metronome app
// Copyright (C) 2022  Pedro Tacla Yamada
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
// = /copyright ===================================================================
//! This module wraps a singleton instance of the standalone audio processor.
//!
//! This instance is held behind a mutex. The metronome handle itself wouldn't need locks, but is
//! currently using a lock here for simplicity. The audio-thread reads directly from its handle
//! without waiting on any locks.

use std::sync::Mutex;

use anyhow::Result;
use lazy_static::lazy_static;

use crate::api::processor::{build_app_processor, AppAudioThreadMessage};
use audio_garbage_collector::Shared;
use audio_processor_metronome::MetronomeProcessorHandle;
use audio_processor_standalone::standalone_processor::StandaloneOptions;
use audio_processor_standalone::{StandaloneAudioOnlyProcessor, StandaloneHandles};

type StandaloneMetronomeHandle = StandaloneHandles;

pub struct State {
    _handles: StandaloneMetronomeHandle,
    pub app_processor_messages: ringbuf::Producer<AppAudioThreadMessage>,
    pub processor_handle: Shared<MetronomeProcessorHandle>,
}

/// The `StandaloneHandles` aren't `Send`. The reason for this is that the `cpal::Stream` isn't
/// `Send`. It should be safe to share this value between threads as long as it can't accessed.
#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for State {}

impl State {
    pub fn new() -> Self {
        let (app_processor_messages, app_processor) = build_app_processor();
        let processor_handle = app_processor.metronome_handle().clone();
        let app = StandaloneAudioOnlyProcessor::new(
            app_processor,
            StandaloneOptions {
                accepts_input: false,
                ..Default::default()
            },
        );
        let handles = audio_processor_standalone::standalone_start_for_env!(app);

        Self {
            app_processor_messages,
            processor_handle,
            _handles: handles,
        }
    }
}

lazy_static! {
    pub static ref STATE: Mutex<Option<State>> = Mutex::new(None);
}

pub fn initialize() {
    wisual_logger::init_from_env();
    let mut state = STATE.lock().unwrap();
    *state = Some(State::new());
}

pub fn deinitialize() {
    let mut handles = STATE.lock().unwrap();
    *handles = None;
}

pub fn with_state0(f: impl FnOnce(&mut State)) -> Result<i32> {
    with_state(|state| {
        f(state);
        Ok(0)
    })
}

pub fn with_state<T>(f: impl FnOnce(&mut State) -> Result<T>) -> Result<T> {
    let mut state = STATE.lock().unwrap();
    if let Some(state) = &mut *state {
        f(state)
    } else {
        Err(anyhow::Error::msg(
            "Failed to lock state. `initialize` needs to be called.",
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    lazy_static! {
        static ref TEST_LOCK: Mutex<()> = Mutex::new(());
    }

    #[test]
    fn test_create_new_state() {
        let _state = State::new();
    }

    #[test]
    fn test_initialize_global_state() {
        let _lock = TEST_LOCK.lock().unwrap();
        initialize();
        let handle = with_state(|state| Ok(state.processor_handle.clone())).unwrap();
        assert_eq!(handle.position_beats(), 0.0);
    }

    #[test]
    fn test_with_state0() {
        let _lock = TEST_LOCK.lock().unwrap();
        initialize();
        let mut was_called = false;
        with_state(|state| {
            let handle = state.processor_handle.clone();
            assert_eq!(handle.position_beats(), 0.0);
            was_called = true;
            Ok(())
        })
        .unwrap();
        assert!(was_called);
    }

    #[test]
    fn test_deinitialize() {
        let _lock = TEST_LOCK.lock().unwrap();
        initialize();
        deinitialize();
    }
}
