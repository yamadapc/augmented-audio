use vst::host::{Host, PluginLoader};
use vst::plugin::Plugin;

#[allow(dead_code)]
pub struct SampleHost;

impl Host for SampleHost {
    fn automate(&self, index: i32, value: f32) {
        println!("Parameter {} had its value changed to {}", index, value);
    }
}

