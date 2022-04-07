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
use serde::Serialize;
use std::error::Error;
use std::ffi::c_void;
use tokio::sync::broadcast::Sender;

pub struct WebviewHolder;

impl WebviewHolder {
    pub unsafe fn new(_size: (i32, i32)) -> WebviewHolder {
        WebviewHolder {}
    }

    pub unsafe fn initialize(&mut self, _parent: *mut c_void, _url: &str) {}

    pub unsafe fn attach_to_parent(&mut self, _parent: *mut c_void) {}
}

impl WebviewHolder {
    pub fn set_on_message_callback(&mut self, _on_message_callback: Sender<String>) {}
    pub fn clear_on_message_callback(&mut self) {}
    pub fn send_message<Msg>(&self, _message: &Msg) -> Result<(), Box<dyn Error>>
    where
        Msg: Serialize,
    {
        todo!()
    }
}
