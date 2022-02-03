use std::sync::Arc;

use vst::api::Events;
use vst::editor::Editor;
use vst::plugin::{Category, HostCallback, Info, Plugin, PluginParameters};
use vst::plugin_main;

use audio_parameter_store::ParameterStore;
use audio_processor_traits::audio_buffer::vst::VSTAudioBuffer;
use audio_processor_traits::midi::vst::midi_slice_from_events;
use audio_processor_traits::{AudioProcessor, AudioProcessorSettings, MidiEventHandler};
use iced_editor::IcedEditor;
use looper_processor::LooperProcessor;

pub use crate::ui::LooperApplication;

pub mod ui;

pub static BUNDLE_IDENTIFIER: &str = "com.beijaflor.Loopi";

pub struct LoopiPlugin {
    parameters: Arc<ParameterStore>,
    processor: LooperProcessor,
    settings: AudioProcessorSettings,
    host_callback: HostCallback,
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

    fn new(host_callback: HostCallback) -> Self
    where
        Self: Sized,
    {
        audio_plugin_logger::init("loopi.log");

        let processor = LooperProcessor::new(audio_garbage_collector::handle());

        LoopiPlugin {
            processor,
            host_callback,
            parameters: Arc::new(ParameterStore::default()),
            settings: AudioProcessorSettings::default(),
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.settings.set_sample_rate(rate);
        self.processor.prepare(self.settings);
    }

    fn set_block_size(&mut self, size: i64) {
        self.settings.set_block_size(size as usize);
        self.processor.prepare(self.settings);
    }

    fn process(&mut self, buffer: &mut vst::buffer::AudioBuffer<f32>) {
        let (inputs, outputs) = buffer.split();
        #[allow(deprecated)]
        let mut vst_buffer = VSTAudioBuffer::new(inputs, outputs);
        self.processor.process(&mut vst_buffer);
    }

    fn process_events(&mut self, events: &Events) {
        self.processor
            .process_midi_events(midi_slice_from_events(events));
    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        self.parameters.clone()
    }

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        Some(Box::new(IcedEditor::<LooperApplication>::new(ui::Flags {
            processor_handle: self.processor.handle(),
            host_callback: Some(self.host_callback.clone()),
        })))
    }
}

impl Drop for LoopiPlugin {
    fn drop(&mut self) {
        log::info!("Loopi is dropped");
    }
}

plugin_main!(LoopiPlugin);
