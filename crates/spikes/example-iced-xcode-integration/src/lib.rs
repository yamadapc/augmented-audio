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

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Text::new("Hello world").into()
    }
}
