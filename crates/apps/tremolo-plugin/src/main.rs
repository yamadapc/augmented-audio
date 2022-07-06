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
use std::sync::Arc;

use audio_processor_standalone::standalone_processor::StandaloneOptions;
use audio_processor_standalone::StandaloneAudioOnlyProcessor;
use tas_v2::build_parameters_editor;
use tas_v2::config::get_configuration_root_path;
use tas_v2::config::logging::configure_logging;
use tas_v2::parameters::build_parameters;
use tas_v2::processor::Processor;

#[cfg(target_os = "macos")]
mod app {
    use std::cell::RefCell;
    use std::ffi::c_void;
    use std::ops::Deref;

    use cacao::layout::Layout;
    use cacao::macos::window::Window;
    use cacao::macos::AppDelegate;
    use cacao::view::View;
    use objc::{msg_send, sel, sel_impl};
    use vst::editor::Editor;

    use generic_parameters_editor::GenericParametersEditor;

    pub struct TremoloApp {
        editor: RefCell<GenericParametersEditor>,
        content_view: View,
        window: Window,
    }

    impl TremoloApp {
        pub fn new(editor: GenericParametersEditor) -> Self {
            Self {
                editor: editor.into(),
                content_view: View::new(),
                window: Window::default(),
            }
        }
    }

    impl AppDelegate for TremoloApp {
        fn did_finish_launching(&self) {
            self.window.set_minimum_content_size(400., 400.);
            self.window.set_title("Hello World!");

            self.window.set_content_view(&self.content_view);
            self.content_view
                .set_translates_autoresizing_mask_into_constraints(true);
            self.window.show();
            self.window.make_key_and_order_front();

            self.content_view.objc.get(|id| unsafe {
                let id = id as *const _ as cocoa::base::id;
                let window_id: cocoa::base::id = msg_send![id, window];
                assert_eq!(self.window.objc.deref() as *const _, window_id);
                self.editor.borrow_mut().open(id as *mut c_void);
            });
        }
    }
}

fn main() {
    let config_root_path = get_configuration_root_path();
    if let Err(err) = configure_logging(&config_root_path) {
        eprintln!("ERROR: Logging set-up has failed {:?}", err);
    }
    log::info!("TremoloPlugin - Started");

    let parameters = Arc::new(build_parameters());
    let processor = Processor::new(parameters.clone());
    let _handles = audio_processor_standalone::standalone_start(StandaloneAudioOnlyProcessor::new(
        processor,
        StandaloneOptions::default(),
    ));

    let editor = build_parameters_editor(&parameters);

    #[cfg(target_os = "macos")]
    cacao::macos::App::new("com.beijaflor.tasv2", app::TremoloApp::new(editor)).run();

    #[cfg(not(target_os = "macos"))]
    std::thread::park();
}
