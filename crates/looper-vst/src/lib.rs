use vst::plugin::{Category, HostCallback, Info, Plugin};
use vst::plugin_main;

use audio_garbage_collector::GarbageCollector;
use audio_processor_traits::audio_buffer::vst::VSTAudioBuffer;
use audio_processor_traits::{AudioProcessor, AudioProcessorSettings};
use looper_processor::LooperProcessor;

struct LoopiPlugin {
    garbage_collector: GarbageCollector,
    processor: LooperProcessor<f32>,
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
        let garbage_collector = GarbageCollector::default();
        let processor = LooperProcessor::new(garbage_collector.handle());

        LoopiPlugin {
            garbage_collector,
            processor,
        }
    }

    fn set_sample_rate(&mut self, rate: f32) {
        // TODO - How to get the right settings into the processor
        self.processor
            .prepare(AudioProcessorSettings::new(rate, 2, 2, 512))
    }

    fn process(&mut self, buffer: &mut vst::buffer::AudioBuffer<f32>) {
        let (inputs, outputs) = buffer.split();
        let mut vst_buffer = VSTAudioBuffer::new(inputs, outputs);
        self.processor.process(&mut vst_buffer);
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
