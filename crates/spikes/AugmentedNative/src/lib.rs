use plugin_host_lib::audio_io::AudioIOService;

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
