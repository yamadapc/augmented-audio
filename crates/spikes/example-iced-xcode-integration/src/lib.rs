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
#[macro_use]
extern crate objc;

use std::ffi::c_void;

use cocoa::base::id;

use baseview::{Size, WindowOpenOptions, WindowScalePolicy};
use iced_baseview::{Application, Command, Element, IcedWindow, Settings, Text};
use window::WindowHolder;

mod macos;
mod window;

#[no_mangle]
pub extern "C" fn attach_to_view(ns_view: id) {
    let window = WindowHolder::new(ns_view as *mut c_void);
    let window_open_options = WindowOpenOptions {
        title: "Iced Editor".to_string(),
        size: Size {
            width: 500.0,
            height: 300.0,
        },
        scale: WindowScalePolicy::SystemScaleFactor,
    };
    let settings = Settings {
        window: window_open_options,
        flags: (),
    };
    IcedWindow::<App>::open_parented(&window, settings);
}

struct App;

impl Application for App {
    type Executor = iced_baseview::executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self, Command::none())
    }

    fn update(
        &mut self,
        _window_queue: &mut iced_baseview::WindowQueue,
        _message: Self::Message,
    ) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Text::new("Hello world").into()
    }
}
