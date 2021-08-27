use std::thread;
use std::time::Duration;

use callbacks::*;
use plugin_host_lib::audio_io::AudioIOService;

// Taken from - https://www.nickwilcox.com/blog/recipe_swift_rust_callback/
mod callbacks {
    use std::ffi::c_void;

    #[repr(C)]
    pub struct CompletedCallback {
        userdata: *mut c_void,
        callback: extern "C" fn(*mut c_void, bool),
    }

    unsafe impl Send for CompletedCallback {}

    impl CompletedCallback {
        pub fn succeeded(self) {
            (self.callback)(self.userdata, true);
            std::mem::forget(self)
        }
        pub fn failed(self) {
            (self.callback)(self.userdata, false);
            std::mem::forget(self)
        }
    }

    impl Drop for CompletedCallback {
        fn drop(&mut self) {
            panic!("CompletedCallback must have explicit succeeded or failed call")
        }
    }
}

#[no_mangle]
pub extern "C" fn async_operation(callback: CompletedCallback) {
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(3));
        callback.succeeded()
    });
}

pub fn initialize_logger() {
    let _ = wisual_logger::try_init_from_env();
}

pub struct AudioGuiInitialModel {
    host_ids: Vec<String>,
    input_ids: Vec<String>,
    output_ids: Vec<String>,
}

pub fn get_audio_info() -> AudioGuiInitialModel {
    log::info!("get_audio_info called");
    let host_list = AudioIOService::hosts();
    let input_list = AudioIOService::input_devices(None).unwrap();
    let output_list = AudioIOService::output_devices(None).unwrap();

    AudioGuiInitialModel {
        host_ids: host_list,
        input_ids: input_list.into_iter().map(|device| device.name).collect(),
        output_ids: output_list.into_iter().map(|device| device.name).collect(),
    }
}

#[derive(Debug)]
pub struct AudioGuiModel {
    host_id: Option<String>,
    input_id: Option<String>,
    output_id: Option<String>,
}

pub fn set_audio_info(model: AudioGuiModel) {
    log::info!("set_audio_info called with {:?}", model);
}

uniffi_macros::include_scaffolding!("augmented");
