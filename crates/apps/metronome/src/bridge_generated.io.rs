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

#![allow(clippy::not_unsafe_ptr_arg_deref)]
use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_initialize(port_: i64) {
    wire_initialize_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_deinitialize(port_: i64) {
    wire_deinitialize_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_set_is_playing(port_: i64, value: bool) {
    wire_set_is_playing_impl(port_, value)
}

#[no_mangle]
pub extern "C" fn wire_set_tempo(port_: i64, value: f32) {
    wire_set_tempo_impl(port_, value)
}

#[no_mangle]
pub extern "C" fn wire_set_volume(port_: i64, value: f32) {
    wire_set_volume_impl(port_, value)
}

#[no_mangle]
pub extern "C" fn wire_set_beats_per_bar(port_: i64, value: i32) {
    wire_set_beats_per_bar_impl(port_, value)
}

#[no_mangle]
pub extern "C" fn wire_set_sound(port_: i64, value: i32) {
    wire_set_sound_impl(port_, value)
}

#[no_mangle]
pub extern "C" fn wire_get_playhead(port_: i64) {
    wire_get_playhead_impl(port_)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

// Section: wire structs

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
