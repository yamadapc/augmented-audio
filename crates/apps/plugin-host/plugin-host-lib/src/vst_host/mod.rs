use std::option::Option::None;

use vst::api::{Events, TimeInfo};
use vst::host::Host;

pub struct AudioTestHost;

impl Host for AudioTestHost {
    // TODO - Maybe if we do something here we'll fix external vsts
    fn automate(&self, index: i32, value: f32) {
        log::debug!("Parameter {} had its value changed to {}", index, value);
    }

    fn begin_edit(&self, _index: i32) {
        log::info!("Begin edit");
    }

    fn end_edit(&self, _index: i32) {
        log::info!("End edit");
    }

    fn get_plugin_id(&self) -> i32 {
        log::info!("Get plugin ID");
        0
    }

    fn idle(&self) {
        log::info!("Idle");
    }

    fn get_info(&self) -> (isize, String, String) {
        log::info!("Get info");
        (1, "Beijaflor Software".to_owned(), "plugin-host".to_owned())
    }

    fn process_events(&self, _events: &Events) {
        log::info!("Events");
    }

    // TODO - Implement tempo
    fn get_time_info(&self, _mask: i32) -> Option<TimeInfo> {
        // log::debug!("Get time info");
        None
    }

    fn get_block_size(&self) -> isize {
        0
    }

    // TODO - Not called by Serum?
    fn update_display(&self) {
        log::info!("Update display");
    }
}
