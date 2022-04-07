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
use cacao::foundation::id;
use cacao::macos::window::Window;
use cacao::macos::{App, AppDelegate};
use cacao::objc::runtime::Object;
use cacao::utils::Controller;
use objc_id::ShareId;

use audiounit::{AUAudioUnit, AVAudioUnitComponentManager};

struct AuViewController(id);

impl Controller for AuViewController {
    fn get_backing_node(&self) -> ShareId<Object> {
        unsafe { ShareId::from_ptr(self.0) }
    }
}

#[derive(Default)]
struct AuGuiApp {
    window: Window,
}

impl AppDelegate for AuGuiApp {
    fn did_finish_launching(&self) {
        App::activate();

        let audio_unit_manager = AVAudioUnitComponentManager::shared();
        let components = audio_unit_manager.all_components();
        let component_with_view = components
            .iter()
            .find(|component| component.has_custom_view())
            .unwrap();

        let audio_unit_descr = component_with_view.audio_component_description();
        let audio_unit = AUAudioUnit::instantiate(audio_unit_descr);
        let view_controller = AuViewController(audio_unit.request_view_controller());

        self.window.set_minimum_content_size(500., 500.);
        self.window.set_title("Content");
        self.window.set_content_view_controller(&view_controller);
        self.window.show();
    }
}

fn main() {
    App::new("com.augmented.augui", AuGuiApp::default()).run()
}
