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

use baseview::{Size, WindowOpenOptions, WindowScalePolicy};
use cocoa::base::id;
use iced_baseview::{widget::Text, Application, Command, Element, Settings};

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
        #[cfg(feature = "opengl")]
        gl_config: None,
    };
    let settings = Settings {
        window: window_open_options,
        flags: (),
        iced_baseview: iced_baseview::settings::IcedBaseviewSettings {
            ignore_non_modifier_keys: false,
            always_redraw: true,
        },
    };
    iced_baseview::open_parented::<App, WindowHolder>(&window, settings);
}

struct App;

impl Application for App {
    type Executor = iced_baseview::executor::Default;
    type Message = ();
    type Theme = iced_baseview::renderer::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self, Command::none())
    }

    fn title(&self) -> String {
        "Example".to_string()
    }

    fn update(
        &mut self,
        _window_queue: &mut iced_baseview::window::WindowQueue,
        _message: Self::Message,
    ) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message, Self::Theme> {
        Text::new("Hello world").into()
    }
}
