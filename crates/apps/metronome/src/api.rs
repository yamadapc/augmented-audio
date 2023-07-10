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

use std::sync::Mutex;
use std::time::Duration;

use anyhow::Result;
use flutter_rust_bridge::StreamSink;
use lazy_static::lazy_static;

pub use crate::api::state::InitializeOptions;

pub use self::processor::MetronomeSoundTypeTag;
use self::state::{with_state, with_state0};

mod processor;
mod state;

lazy_static! {
    pub static ref IS_RUNNING: Mutex<bool> = Mutex::new(false);
}

pub fn initialize(options: InitializeOptions) -> Result<i32> {
    let mut is_running = IS_RUNNING
        .lock()
        .map_err(|_| anyhow::format_err!("Running Lock is poisoned"))?;
    *is_running = true;
    state::initialize(options);
    Ok(0)
}

pub fn deinitialize() -> Result<i32> {
    let mut is_running = IS_RUNNING
        .lock()
        .map_err(|_| anyhow::format_err!("Running Lock is poisoned"))?;
    (*is_running) = false;
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

pub fn get_playhead(sink: StreamSink<f32>) -> Result<i32> {
    with_state(|state| {
        let handle = state.processor_handle.clone();
        std::thread::spawn(move || {
            log::info!("Starting streaming of playhead");

            let run = || -> anyhow::Result<()> {
                loop {
                    let is_running = IS_RUNNING
                        .lock()
                        .map_err(|_| anyhow::format_err!("Running Lock is poisoned"))?;
                    if !*is_running {
                        break;
                    }
                    sink.add(handle.position_beats());
                    std::thread::sleep(Duration::from_millis(50));
                }

                Ok(())
            };

            if let Err(err) = run() {
                log::error!("Failure listening to play-head: {:?}", err);
            }
            log::info!("Stream closed, stopping streaming of play-head");
        });
        Ok(0)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_initialize() {
        initialize(Default::default()).unwrap();
    }

    #[test]
    fn test_deinitialize() {
        initialize(Default::default()).unwrap();
        deinitialize().unwrap();
    }

    #[test]
    fn test_set_is_playing() {
        initialize(Default::default()).unwrap();
        let handle = with_state(|state| Ok(state.processor_handle.clone())).unwrap();
        set_is_playing(true).unwrap();
        assert_eq!(handle.is_playing(), true);
        set_is_playing(false).unwrap();
        assert_eq!(handle.is_playing(), false);
    }

    #[test]
    fn test_set_beats_per_bar() {
        initialize(Default::default()).unwrap();
        let handle = with_state(|state| Ok(state.processor_handle.clone())).unwrap();
        set_beats_per_bar(5).unwrap();
        assert_eq!(handle.beats_per_bar(), 5);
        set_beats_per_bar(6).unwrap();
        assert_eq!(handle.beats_per_bar(), 6);
    }

    #[test]
    fn test_set_volume() {
        initialize(Default::default()).unwrap();
        let handle = with_state(|state| Ok(state.processor_handle.clone())).unwrap();
        set_volume(0.44).unwrap();
        assert_eq!(handle.volume(), 0.44);
    }
}
