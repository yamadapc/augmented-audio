// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr::null_mut;

use crate::c_api::into_ptr;
use crate::controllers::audio_state_controller::AudioDevice;
use crate::LooperEngine;

#[no_mangle]
pub unsafe extern "C" fn audio_device__name(device: *mut AudioDevice) -> *mut c_char {
    let device = &(*device);
    CString::new(device.name.clone())
        .unwrap_or_else(|_| CString::default())
        .into_raw()
}

#[repr(C)]
pub struct CAudioDeviceList {
    inner: *mut Vec<*mut AudioDevice>,
}

#[no_mangle]
pub unsafe extern "C" fn audio_device_list__count(device_list: *mut CAudioDeviceList) -> usize {
    let device_list = &(*device_list);
    if device_list.inner.is_null() {
        0
    } else {
        let inner = &(*device_list.inner);
        inner.len()
    }
}

#[no_mangle]
pub unsafe extern "C" fn audio_device_list__get(
    device_list: *mut CAudioDeviceList,
    index: usize,
) -> *mut AudioDevice {
    if device_list.is_null() {
        return null_mut();
    }

    let device_list = &(*device_list);
    if device_list.inner.is_null() {
        null_mut()
    } else {
        let inner = &(*device_list.inner);
        inner.get(index).cloned().unwrap_or(null_mut())
    }
}

#[no_mangle]
pub unsafe extern "C" fn audio_device_list__free(device_list: *mut CAudioDeviceList) {
    for device in &(*(*device_list).inner) {
        let _ = Box::from_raw(*device);
    }
    let _ = Box::from_raw(device_list);
}

#[no_mangle]
pub unsafe extern "C" fn audio_io_settings_controller__list_input_devices(
    engine: *const LooperEngine,
) -> *mut CAudioDeviceList {
    let controller = (*engine).audio_io_settings_controller();
    let devices = controller.list_input_devices();
    into_ptr(into_c_model(devices))
}

#[no_mangle]
pub unsafe extern "C" fn audio_io_settings_controller__list_output_devices(
    engine: *const LooperEngine,
) -> *mut CAudioDeviceList {
    let controller = (*engine).audio_io_settings_controller();
    let devices = controller.list_output_devices();
    into_ptr(into_c_model(devices))
}

#[no_mangle]
pub unsafe extern "C" fn audio_io_settings_controller__get_input_device(
    engine: *const LooperEngine,
) -> *mut c_char {
    let controller = (*engine).audio_io_settings_controller();
    CString::new(controller.input_device())
        .unwrap_or_else(|_| CString::default())
        .into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn audio_io_settings_controller__get_output_device(
    engine: *const LooperEngine,
) -> *mut c_char {
    let controller = (*engine).audio_io_settings_controller();
    CString::new(controller.output_device())
        .unwrap_or_else(|_| CString::default())
        .into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn audio_io_settings_controller__set_input_device(
    engine: *const LooperEngine,
    device: *const c_char,
) {
    let controller = (*engine).audio_io_settings_controller();
    let device = CStr::from_ptr(device).to_str().unwrap_or("");
    controller.set_input_device(device);
}

#[no_mangle]
pub unsafe extern "C" fn audio_io_settings_controller__set_output_device(
    engine: *const LooperEngine,
    device: *const c_char,
) {
    let controller = (*engine).audio_io_settings_controller();
    let device = CStr::from_ptr(device).to_str().unwrap_or("");
    controller.set_output_device(device);
}

fn into_c_model(devices: anyhow::Result<Vec<AudioDevice>>) -> CAudioDeviceList {
    match devices {
        Ok(device) => {
            let inner = into_ptr(device.into_iter().map(into_ptr).collect());
            CAudioDeviceList { inner }
        }
        Err(err) => {
            log::error!("Failed to list input devices: {}", err);
            CAudioDeviceList { inner: null_mut() }
        }
    }
}
