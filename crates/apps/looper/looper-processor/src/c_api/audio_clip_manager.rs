use std::ffi::CStr;
use std::os::raw::c_char;

use crate::services::audio_clip_manager::LoadClipMessage;
use crate::LooperEngine;

#[no_mangle]
pub unsafe extern "C" fn looper_engine__load_file(
    engine: *mut LooperEngine,
    file_path: *const c_char,
) {
    let engine = &(*engine);
    let manager = engine.audio_clip_manager();
    let file_path = CStr::from_ptr(file_path).to_str().unwrap().to_string();
    manager.do_send(LoadClipMessage {
        path: file_path.into(),
    });
}
