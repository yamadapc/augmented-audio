use vst::host::Host;

pub struct AudioTestHost;

impl Host for AudioTestHost {
    fn automate(&self, index: i32, value: f32) {
        println!("Parameter {} had its value changed to {}", index, value);
    }
}
