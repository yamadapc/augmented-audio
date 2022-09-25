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

use augmented::audio::gc::{make_shared, Shared};
use augmented::vst::buffer::AudioBuffer;
use augmented::vst::editor::Editor;
use augmented::vst::plugin::{Category, HostCallback, Info, Plugin};
use augmented::vst::plugin_main;

mod view;

struct HostTempoPlugin {
    host_callback: Shared<HostCallback>,
}

impl Plugin for HostTempoPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "HostTempoPlugin".to_string(),
            category: Category::Effect,
            vendor: "Beijaflor".to_string(),
            unique_id: 3390, // Used by hosts to differentiate between plugins.
            version: 1_3_3,
            parameters: 0,
            ..Default::default()
        }
    }

    fn new(host: HostCallback) -> Self {
        augmented::ops::audio_plugin_logger::init("tempo-sync.log");

        HostTempoPlugin {
            host_callback: make_shared(host),
        }
    }

    fn process(&mut self, _buffer: &mut AudioBuffer<f32>) {}

    fn start_process(&mut self) {
        log::info!("start_process");
    }

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        Some(view::get_editor(self.host_callback.clone()))
    }
}

impl Drop for HostTempoPlugin {
    fn drop(&mut self) {
        log::info!("Shutting-down plugin");
    }
}

plugin_main!(HostTempoPlugin);
