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

use std::time::Duration;

use anyhow::Result;
use flutter_rust_bridge::StreamSink;

use state::{with_state, with_state0};

mod state;

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_initialize() {
        initialize().unwrap();
    }

    #[test]
    fn test_deinitialize() {
        initialize().unwrap();
        deinitialize().unwrap();
    }

    #[test]
    fn test_set_is_playing() {
        initialize().unwrap();
        let handle = with_state(|state| Ok(state.processor_handle.clone())).unwrap();
        set_is_playing(true).unwrap();
        assert_eq!(handle.is_playing(), true);
        set_is_playing(false).unwrap();
        assert_eq!(handle.is_playing(), false);
    }

    #[test]
    fn test_set_beats_per_bar() {
        initialize().unwrap();
        let handle = with_state(|state| Ok(state.processor_handle.clone())).unwrap();
        set_beats_per_bar(5).unwrap();
        assert_eq!(handle.beats_per_bar(), 5);
        set_beats_per_bar(6).unwrap();
        assert_eq!(handle.beats_per_bar(), 6);
    }

    #[test]
    fn test_set_volume() {
        initialize().unwrap();
        let handle = with_state(|state| Ok(state.processor_handle.clone())).unwrap();
        set_volume(0.44).unwrap();
        assert_eq!(handle.volume(), 0.44);
    }
}
