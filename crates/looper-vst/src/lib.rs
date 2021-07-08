use std::sync::Arc;

use vst::api::Events;
use vst::editor::Editor;
use vst::plugin::{Category, HostCallback, Info, Plugin};
use vst::plugin_main;

use audio_garbage_collector::GarbageCollector;
use audio_parameter_store::ParameterStore;
use audio_processor_traits::audio_buffer::vst::VSTAudioBuffer;
use audio_processor_traits::{AudioProcessor, AudioProcessorSettings, MidiEventHandler};
use iced_editor::IcedEditor;
use looper_processor::LooperProcessor;

use crate::ui::LooperApplication;

mod config;
mod ui;

pub static BUNDLE_IDENTIFIER: &str = "com.beijaflor.Loopi";
pub static INDEX_HTML_RESOURCE: &str = "frontend/index.html";

struct LoopiPlugin {
    garbage_collector: GarbageCollector,
    parameters: Arc<ParameterStore>,
    processor: LooperProcessor<f32>,
    settings: AudioProcessorSettings,
}

impl Plugin for LoopiPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "Loopi".to_string(),
            category: Category::Effect,
            vendor: "Beijaflor Software".to_string(),
            unique_id: 2502, // Used by hosts to differentiate between plugins.
            parameters: 0,
            ..Default::default()
        }
    }

    fn new(_host: HostCallback) -> Self
    where
        Self: Sized,
    {
        let _ = config::logging::configure_logging(&config::get_configuration_root_path());

        let garbage_collector = GarbageCollector::default();
        let processor = LooperProcessor::new(garbage_collector.handle());

        LoopiPlugin {
            garbage_collector,
            processor,
            parameters: Arc::new(ParameterStore::default()),
            settings: AudioProcessorSettings::default(),
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        self.settings.set_sample_rate(rate);
        self.processor.prepare(self.settings);
    }

    fn set_block_size(&mut self, size: i64) {
        self.settings.set_block_size(size as u32);
        self.processor.prepare(self.settings);
    }

    fn process(&mut self, buffer: &mut vst::buffer::AudioBuffer<f32>) {
        let (inputs, outputs) = buffer.split();
        let mut vst_buffer = VSTAudioBuffer::new(inputs, outputs);
        self.processor.process(&mut vst_buffer);
    }

    fn process_events(&mut self, events: &Events) {
        self.processor.process_midi_events(unsafe {
            std::slice::from_raw_parts(
                &events.events[0] as *const *mut _,
                events.num_events as usize,
            )
        });
    }

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        Some(Box::new(IcedEditor::<LooperApplication>::new(())))
    }
}

impl Drop for LoopiPlugin {
    fn drop(&mut self) {
        if let Err(err) = self.garbage_collector.stop() {
            log::error!("Failed to stop GC {}", err);
        }
    }
}

plugin_main!(LoopiPlugin);
