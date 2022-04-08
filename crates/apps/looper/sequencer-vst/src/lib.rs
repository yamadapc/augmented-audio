use audio_parameter_store::ParameterStore;
use audio_processor_traits::audio_buffer::vst::VSTAudioBuffer;
use audio_processor_traits::{AudioProcessor, AudioProcessorSettings, MidiEventHandler};
use iced_editor::IcedEditor;
use looper_processor::{LooperEngine, LooperOptions, MultiTrackLooper};
use std::ffi::c_void;
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
use audio_processor_traits::midi::vst::midi_slice_from_events;
use looper_processor::engine::{AudioModeParams, LooperEngineParams};
use std::sync::Arc;
use vst::api::Events;
use vst::editor::Editor;
use vst::plugin::{Category, HostCallback, Info, Plugin, PluginParameters};
use vst::plugin_main;

pub use crate::ui::LooperApplication;

pub static BUNDLE_IDENTIFIER: &str = "com.beijaflor.SequencerVST";

pub struct SequencerPlugin {
    engine: Arc<LooperEngine>,
    settings: AudioProcessorSettings,
}

impl Plugin for SequencerPlugin {
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

    fn new(host_callback: HostCallback) -> Self
    where
        Self: Sized,
    {
        audio_plugin_logger::init("loopi.log");

        let engine = Arc::new(LooperEngine::new(LooperEngineParams {
            audio_mode: AudioModeParams::Hosted(Some(host_callback)),
        }));

        LoopiPlugin {
            engine,
            settings: AudioProcessorSettings::default(),
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.settings.set_sample_rate(rate);
        self.engine.processor().unwrap().prepare(self.settings);
    }

    fn set_block_size(&mut self, size: i64) {
        self.settings.set_block_size(size as usize);
        self.engine.processor().unwrap().prepare(self.settings);
    }

    fn process(&mut self, buffer: &mut vst::buffer::AudioBuffer<f32>) {
        let (inputs, outputs) = buffer.split();
        #[allow(deprecated)]
        let mut vst_buffer = VSTAudioBuffer::new(inputs, outputs);
        self.engine.processor().unwrap().process(&mut vst_buffer);
    }

    fn process_events(&mut self, _events: &Events) {
        self.engine
            .processor()
            .unwrap()
            .process_midi_events(midi_slice_from_events(events));
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        self.parameters.clone()
    }

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        Some(Box::new(SequencerEditor {
            engine: self.engine.clone(),
        }))
    }
}

struct SequencerEditor {
    engine: Arc<LooperEngine>,
}

impl Editor for SequencerEditor {
    fn size(&self) -> (i32, i32) {
        (1000, 900)
    }

    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn open(&mut self, parent: *mut c_void) -> bool {}

    fn is_open(&mut self) -> bool {}
}

plugin_main!(LoopiPlugin);
