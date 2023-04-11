use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_initialize(port_: i64, options: *mut wire_InitializeOptions) {
    wire_initialize_impl(port_, options)
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

#[no_mangle]
pub extern "C" fn new_box_autoadd_initialize_options_0() -> *mut wire_InitializeOptions {
    support::new_leak_box_ptr(wire_InitializeOptions::new_with_null_ptr())
}

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

impl Wire2Api<InitializeOptions> for *mut wire_InitializeOptions {
    fn wire2api(self) -> InitializeOptions {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<InitializeOptions>::wire2api(*wrap).into()
    }
}

impl Wire2Api<InitializeOptions> for wire_InitializeOptions {
    fn wire2api(self) -> InitializeOptions {
        InitializeOptions {
            assets_file_path: self.assets_file_path.wire2api(),
        }
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
pub struct wire_InitializeOptions {
    assets_file_path: *mut wire_uint_8_list,
}

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

impl NewWithNullPtr for wire_InitializeOptions {
    fn new_with_null_ptr() -> Self {
        Self {
            assets_file_path: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_InitializeOptions {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
