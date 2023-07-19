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
//! This module contains the public API exposed to flutter

use anyhow::Result;
use flutter_rust_bridge::StreamSink;

pub use crate::api::state::InitializeOptions;

pub use self::processor::MetronomeSoundTypeTag;
use self::state::{with_state, with_state0};

mod processor;
mod state;

pub fn initialize(options: InitializeOptions) -> Result<i32> {
    state::initialize(options);
    Ok(0)
}

pub fn deinitialize() -> Result<i32> {
    log::info!("Deinitializing state");
    state::deinitialize();
    log::info!("Shutdown sequence complete");
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

pub fn set_sound(value: MetronomeSoundTypeTag) -> Result<i32> {
    with_state0(|state| {
        log::info!("set_sound({:?})", value);
        if state
            .app_processor_messages
            .push(self::processor::AppAudioThreadMessage::SetMetronomeSound(
                value,
            ))
            .is_err()
        {
            log::error!("Message queue is full!");
        }
    })
}

pub fn get_playhead() -> Result<f32> {
    with_state(|state| {
        let handle = state.processor_handle.clone();
        Ok(handle.position_beats())
    })
}

#[derive(Debug)]
#[repr(C)]
pub struct EngineError {
    pub message: String,
}

pub fn stream_errors(stream: StreamSink<EngineError>) {
    loop {
        let rx = with_state(|state| {
            state
                .handles
                .errors_rx()
                .recv()
                .map_err(anyhow::Error::from)
        });

        if let Ok(error) = rx {
            if !stream.add(EngineError {
                message: error.to_string(),
            }) {
                break;
            }
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod test {
    use lazy_static::lazy_static;
    use std::sync::Mutex;

    use super::*;

    lazy_static! {
        static ref TEST_LOCK: Mutex<()> = Mutex::new(());
    }

    #[test]
    fn test_initialize() {
        initialize(Default::default()).unwrap();
    }

    #[test]
    fn test_deinitialize() {
        let _lock = TEST_LOCK.lock().unwrap();
        initialize(Default::default()).unwrap();
        deinitialize().unwrap();
    }

    #[test]
    fn test_set_is_playing() {
        let _lock = TEST_LOCK.lock().unwrap();
        initialize(Default::default()).unwrap();
        let handle = with_state(|state| Ok(state.processor_handle.clone())).unwrap();
        set_is_playing(true).unwrap();
        assert_eq!(handle.is_playing(), true);
        set_is_playing(false).unwrap();
        assert_eq!(handle.is_playing(), false);
    }

    #[test]
    fn test_set_beats_per_bar() {
        let _lock = TEST_LOCK.lock().unwrap();
        initialize(Default::default()).unwrap();
        let handle = with_state(|state| Ok(state.processor_handle.clone())).unwrap();
        set_beats_per_bar(5).unwrap();
        assert_eq!(handle.beats_per_bar(), 5);
        set_beats_per_bar(6).unwrap();
        assert_eq!(handle.beats_per_bar(), 6);
    }

    #[test]
    fn test_set_volume() {
        let _lock = TEST_LOCK.lock().unwrap();
        initialize(Default::default()).unwrap();
        let handle = with_state(|state| Ok(state.processor_handle.clone())).unwrap();
        set_volume(0.44).unwrap();
        assert_eq!(handle.volume(), 0.44);
    }
}
