#[macro_use]
extern crate vst;

use vst::plugin::{HostCallback, Info, Plugin, Category};

struct BasicPlugin;

impl Plugin for BasicPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "TasV2".to_string(),
            category: Category::Effect,
            vendor: "Beijaflor Software".to_string(),
            unique_id: 2501, // Used by hosts to differentiate between plugins.
            ..Default::default()
        }
    }

    fn new(_host: HostCallback) -> Self {
        BasicPlugin
    }
}

plugin_main!(BasicPlugin); // Important!
