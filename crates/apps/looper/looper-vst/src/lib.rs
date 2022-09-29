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

use vst::api::Events;
use vst::editor::Editor;
use vst::plugin::{Category, HostCallback, Info, Plugin, PluginParameters};
use vst::plugin_main;

use audio_parameter_store::ParameterStore;
use audio_processor_traits::audio_buffer::vst::VSTBufferHandler;
use audio_processor_traits::{AudioProcessor, AudioProcessorSettings, MidiEventHandler};
use augmented_iced_editor::IcedEditor;
use looper_processor::{LooperOptions, MultiTrackLooper};

pub use crate::ui::LooperApplication;

pub mod ui;

pub static BUNDLE_IDENTIFIER: &str = "com.beijaflor.Loopi";

pub struct LoopiPlugin {
    parameters: Arc<ParameterStore>,
    processor: MultiTrackLooper,
    settings: AudioProcessorSettings,
    buffer_handler: VSTBufferHandler<f32>,
}

impl Plugin for LoopiPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "Loopi".to_string(),
            category: Category::Effect,
            vendor: "Beijaflor Software".to_string(),
            unique_id: 2504, // Used by hosts to differentiate between plugins.
            parameters: 0,
            ..Default::default()
        }
    }

    fn new(_host_callback: HostCallback) -> Self
    where
        Self: Sized,
    {
        audio_plugin_logger::init("loopi.log");

        let processor = MultiTrackLooper::new(
            LooperOptions {
                ..Default::default()
            },
            1,
        );

        LoopiPlugin {
            processor,
            parameters: Arc::new(ParameterStore::default()),
            settings: AudioProcessorSettings::default(),
            buffer_handler: Default::default(),
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.settings.set_sample_rate(rate);
        self.processor.prepare(self.settings);
    }

    fn set_block_size(&mut self, size: i64) {
        self.settings.set_block_size(size as usize);
        self.processor.prepare(self.settings);
        self.buffer_handler.set_block_size(size as usize);
    }

    fn process(&mut self, vst_buffer: &mut vst::buffer::AudioBuffer<f32>) {
        let processor = &mut self.processor;
        self.buffer_handler.with_buffer(vst_buffer, |buffer| {
            processor.process(buffer);
        });
    }

    fn process_events(&mut self, events: &Events) {
        self.processor.process_midi_events(
            audio_processor_traits::midi::vst::midi_slice_from_events(events),
        );
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        self.parameters.clone()
    }

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        Some(Box::new(IcedEditor::<LooperApplication>::new_with(
            ui::Flags {
                processor_handle: self.processor.handle().clone(),
            },
            (700, 300),
        )))
    }
}

impl Drop for LoopiPlugin {
    fn drop(&mut self) {
        log::info!("Loopi is dropped");
    }
}

plugin_main!(LoopiPlugin);
