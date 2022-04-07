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
use std::option::Option::None;

use vst::api::{Events, TimeInfo};
use vst::host::Host;

pub struct AudioTestHost;

impl Host for AudioTestHost {
    // TODO - Maybe if we do something here we'll fix external vsts
    fn automate(&self, index: i32, value: f32) {
        log::debug!("Parameter {} had its value changed to {}", index, value);
    }

    fn begin_edit(&self, _index: i32) {
        log::info!("Begin edit");
    }

    fn end_edit(&self, _index: i32) {
        log::info!("End edit");
    }

    fn get_plugin_id(&self) -> i32 {
        log::info!("Get plugin ID");
        0
    }

    fn idle(&self) {
        log::info!("Idle");
    }

    fn get_info(&self) -> (isize, String, String) {
        log::info!("Get info");
        (1, "Beijaflor Software".to_owned(), "plugin-host".to_owned())
    }

    fn process_events(&self, _events: &Events) {
        log::info!("Events");
    }

    // TODO - Implement tempo
    fn get_time_info(&self, _mask: i32) -> Option<TimeInfo> {
        // log::debug!("Get time info");
        None
    }

    fn get_block_size(&self) -> isize {
        0
    }

    // TODO - Not called by Serum?
    fn update_display(&self) {
        log::info!("Update display");
    }
}
