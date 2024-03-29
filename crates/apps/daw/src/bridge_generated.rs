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

#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.62.0.

use crate::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_initialize_logger_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "initialize_logger",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| initialize_logger(),
    )
}
fn wire_initialize_audio_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "initialize_audio",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| initialize_audio(),
    )
}
fn wire_start_playback_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "start_playback",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| start_playback(),
    )
}
fn wire_stop_playback_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "stop_playback",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| stop_playback(),
    )
}
fn wire_set_vst_file_path_impl(port_: MessagePort, path: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_vst_file_path",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_path = path.wire2api();
            move |task_callback| set_vst_file_path(api_path)
        },
    )
}
fn wire_set_input_file_path_impl(port_: MessagePort, path: impl Wire2Api<String> + UnwindSafe) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "set_input_file_path",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_path = path.wire2api();
            move |task_callback| set_input_file_path(api_path)
        },
    )
}
fn wire_audio_io_get_input_devices_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "audio_io_get_input_devices",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| audio_io_get_input_devices(),
    )
}
fn wire_get_events_sink_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_events_sink",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || move |task_callback| get_events_sink(task_callback.stream_sink()),
    )
}
fn wire_audio_thread_set_options_impl(
    port_: MessagePort,
    input_device_id: impl Wire2Api<String> + UnwindSafe,
    output_device_id: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "audio_thread_set_options",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_input_device_id = input_device_id.wire2api();
            let api_output_device_id = output_device_id.wire2api();
            move |task_callback| audio_thread_set_options(api_input_device_id, api_output_device_id)
        },
    )
}
fn wire_audio_graph_setup_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "audio_graph_setup",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| audio_graph_setup(),
    )
}
fn wire_audio_graph_get_system_indexes_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "audio_graph_get_system_indexes",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| audio_graph_get_system_indexes(),
    )
}
fn wire_audio_graph_connect_impl(
    port_: MessagePort,
    input_index: impl Wire2Api<u32> + UnwindSafe,
    output_index: impl Wire2Api<u32> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "audio_graph_connect",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_input_index = input_index.wire2api();
            let api_output_index = output_index.wire2api();
            move |task_callback| audio_graph_connect(api_input_index, api_output_index)
        },
    )
}
fn wire_audio_node_create_impl(
    port_: MessagePort,
    audio_processor_name: impl Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "audio_node_create",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_audio_processor_name = audio_processor_name.wire2api();
            move |task_callback| audio_node_create(api_audio_processor_name)
        },
    )
}
fn wire_audio_node_set_parameter_impl(
    port_: MessagePort,
    _audio_node_id: impl Wire2Api<i32> + UnwindSafe,
    _parameter_name: impl Wire2Api<String> + UnwindSafe,
    _parameter_value: impl Wire2Api<f32> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "audio_node_set_parameter",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api__audio_node_id = _audio_node_id.wire2api();
            let api__parameter_name = _parameter_name.wire2api();
            let api__parameter_value = _parameter_value.wire2api();
            move |task_callback| {
                audio_node_set_parameter(
                    api__audio_node_id,
                    api__parameter_name,
                    api__parameter_value,
                )
            }
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<f32> for f32 {
    fn wire2api(self) -> f32 {
        self
    }
}
impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}
impl Wire2Api<u32> for u32 {
    fn wire2api(self) -> u32 {
        self
    }
}
impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
