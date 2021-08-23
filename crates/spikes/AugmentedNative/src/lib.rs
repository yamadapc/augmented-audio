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
    println!("get_audio_info");
    let host_list = AudioIOService::hosts();
    println!("get_audio_info - hosts");
    let input_list = AudioIOService::input_devices(None).unwrap();
    println!("get_audio_info - inputs");
    let output_list = AudioIOService::output_devices(None).unwrap();
    println!("get_audio_info - outputs");

    AudioGuiInitialModel {
        host_ids: host_list,
        input_ids: input_list.into_iter().map(|device| device.name).collect(),
        output_ids: output_list.into_iter().map(|device| device.name).collect(),
    }
}

uniffi_macros::include_scaffolding!("augmented");
