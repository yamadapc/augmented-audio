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

use std::ffi::CStr;
use std::os::raw::c_char;

use actix_system_threads::ActorSystem;

use crate::services::audio_clip_manager::LoadClipMessage;
use crate::LooperEngine;

#[no_mangle]
pub unsafe extern "C" fn looper_engine__load_file(
    engine: *const LooperEngine,
    file_path: *const c_char,
) {
    let engine = &(*engine);
    let manager = engine.audio_clip_manager();
    let file_path = CStr::from_ptr(file_path).to_str().unwrap().to_string();
    ActorSystem::current().spawn_result(async move {
        // TODO handle errors
        let _ = manager
            .send(LoadClipMessage {
                path: file_path.into(),
            })
            .await;
    });
}
