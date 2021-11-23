use plugin_host_lib::audio_io::AudioIOService;

#[allow(dead_code)]
pub struct AvailableAudioOptions {
    pub host_ids: Vec<String>,
    pub input_ids: Vec<String>,
    pub output_ids: Vec<String>,
}

#[derive(Debug)]
pub struct AudioOptions {
    pub host_id: Option<String>,
    pub input_id: Option<String>,
}

pub struct AudioOptionsService {}

impl Default for AudioOptionsService {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioOptionsService {
    pub fn new() -> Self {
        Self {}
    }

    #[allow(dead_code)]
    pub fn get_available_options(&self) -> AvailableAudioOptions {
        log::info!("get_audio_info called");
        let host_list = AudioIOService::hosts();
        let input_list = AudioIOService::input_devices(None).unwrap();
        let output_list = AudioIOService::output_devices(None).unwrap();

        AvailableAudioOptions {
            host_ids: host_list,
            input_ids: input_list.into_iter().map(|device| device.name).collect(),
            output_ids: output_list.into_iter().map(|device| device.name).collect(),
        }
    }

    #[allow(dead_code)]
    pub fn set_options(&self, model: AudioOptions) {
        log::info!("set_audio_info called with {:?}", model);
    }
}
