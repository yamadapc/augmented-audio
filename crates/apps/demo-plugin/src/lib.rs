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
use crate::processor::ProcessorHandleRef;
use augmented::vst::buffer::AudioBuffer;
use augmented::vst::editor::Editor;
use augmented::vst::plugin::{Category, HostCallback, Info, Plugin, PluginParameters};
use augmented::vst::plugin_main;
use processor::Processor;
use std::sync::Arc;

pub mod processor;

struct DemoPlugin {
    processor: Processor,
}

impl Plugin for DemoPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "Demo".to_string(),
            category: Category::Effect,
            vendor: "Beijaflor Software".to_string(),
            unique_id: 2501, // Used by hosts to differentiate between plugins.
            parameters: 1,
            ..Default::default()
        }
    }

    fn new(_host: HostCallback) -> Self {
        augmented::ops::audio_plugin_logger::init("demo.log");
        let processor = Processor::new();

        DemoPlugin { processor }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        log::info!("set_sample_rate: {}", rate);
        self.processor.set_sample_rate(rate);
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        self.processor.process(buffer);
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        let shared: ProcessorHandleRef = self.processor.handle();
        Arc::new(shared)
    }

    fn start_process(&mut self) {
        log::info!("start_process");
    }

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        None
    }
}

impl Drop for DemoPlugin {
    fn drop(&mut self) {
        log::info!("Shutting-down plugin");
    }
}

plugin_main!(DemoPlugin);
