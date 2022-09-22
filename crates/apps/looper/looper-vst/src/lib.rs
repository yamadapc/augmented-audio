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

use audio_processor_traits::{
    AudioBuffer, AudioProcessor, AudioProcessorSettings, OwnedAudioBuffer, VecAudioBuffer,
};
use iced_editor::IcedEditor;
use looper_processor::{LooperOptions, MultiTrackLooper};

pub use crate::ui::LooperApplication;

pub mod ui;

pub static BUNDLE_IDENTIFIER: &str = "com.beijaflor.Loopi";

pub struct LoopiPlugin {
    parameters: Arc<ParameterStore>,
    processor: MultiTrackLooper,
    settings: AudioProcessorSettings,
    buffer: VecAudioBuffer<f32>,
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
            buffer: VecAudioBuffer::new(),
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.settings.set_sample_rate(rate);
        self.processor.prepare(self.settings);
    }

    fn set_block_size(&mut self, size: i64) {
        self.settings.set_block_size(size as usize);
        self.processor.prepare(self.settings);
        self.buffer.resize(2, size as usize, 0.0);
    }

    fn process(&mut self, buffer: &mut vst::buffer::AudioBuffer<f32>) {
        let num_samples = buffer.samples();
        let (inputs, mut outputs) = buffer.split();
        // TODO extract this
        self.buffer.resize(2, num_samples as usize, 0.0);
        {
            let buffer_slice = self.buffer.slice_mut();
            for (channel, input) in inputs.into_iter().take(2).enumerate() {
                for (index, sample) in input.iter().enumerate() {
                    buffer_slice[index * 2 + channel] = *sample;
                }
            }
        }

        self.processor.process(&mut self.buffer);

        {
            let buffer_slice = self.buffer.slice();
            for (channel, output) in outputs.into_iter().take(2).enumerate() {
                for (index, sample) in output.iter_mut().enumerate() {
                    *sample = buffer_slice[index * 2 + channel];
                }
            }
        }
    }

    fn process_events(&mut self, _events: &Events) {
        // self.processor
        //     .process_midi_events(midi_slice_from_events(events));
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
