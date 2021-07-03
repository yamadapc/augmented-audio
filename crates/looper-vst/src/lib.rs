use vst::plugin::{Category, HostCallback, Info, Plugin};
use vst::plugin_main;

struct LoopiPlugin {}

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
        LoopiPlugin {}
    }

    fn set_sample_rate(&mut self, rate: f32) {
        log::info!("LoopiPlugin::set_sample_rate: {}", rate);
    }

    fn process(&mut self, _buffer: &mut vst::buffer::AudioBuffer<f32>) {}
}

plugin_main!(LoopiPlugin);
