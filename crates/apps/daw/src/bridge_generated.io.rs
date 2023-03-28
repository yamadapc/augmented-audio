// = copyright ====================================================================
// DAW: Flutter UI for a DAW application
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

use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_initialize_logger(port_: i64) {
    wire_initialize_logger_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_initialize_audio(port_: i64) {
    wire_initialize_audio_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_start_playback(port_: i64) {
    wire_start_playback_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_stop_playback(port_: i64) {
    wire_stop_playback_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_set_vst_file_path(port_: i64, path: *mut wire_uint_8_list) {
    wire_set_vst_file_path_impl(port_, path)
}

#[no_mangle]
pub extern "C" fn wire_set_input_file_path(port_: i64, path: *mut wire_uint_8_list) {
    wire_set_input_file_path_impl(port_, path)
}

#[no_mangle]
pub extern "C" fn wire_audio_io_get_input_devices(port_: i64) {
    wire_audio_io_get_input_devices_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_events_sink(port_: i64) {
    wire_get_events_sink_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_audio_thread_set_options(
    port_: i64,
    input_device_id: *mut wire_uint_8_list,
    output_device_id: *mut wire_uint_8_list,
) {
    wire_audio_thread_set_options_impl(port_, input_device_id, output_device_id)
}

#[no_mangle]
pub extern "C" fn wire_audio_graph_setup(port_: i64) {
    wire_audio_graph_setup_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_audio_graph_get_system_indexes(port_: i64) {
    wire_audio_graph_get_system_indexes_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_audio_graph_connect(port_: i64, input_index: u32, output_index: u32) {
    wire_audio_graph_connect_impl(port_, input_index, output_index)
}

#[no_mangle]
pub extern "C" fn wire_audio_node_create(port_: i64, audio_processor_name: *mut wire_uint_8_list) {
    wire_audio_node_create_impl(port_, audio_processor_name)
}

#[no_mangle]
pub extern "C" fn wire_audio_node_set_parameter(
    port_: i64,
    _audio_node_id: i32,
    _parameter_name: *mut wire_uint_8_list,
    _parameter_value: f32,
) {
    wire_audio_node_set_parameter_impl(port_, _audio_node_id, _parameter_name, _parameter_value)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

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
