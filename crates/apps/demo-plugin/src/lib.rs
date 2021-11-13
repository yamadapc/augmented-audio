use augmented::vst::buffer::AudioBuffer;
use augmented::vst::editor::Editor;
use augmented::vst::plugin::{Category, HostCallback, Info, Plugin};
use augmented::vst::plugin_main;
use processor::Processor;

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
