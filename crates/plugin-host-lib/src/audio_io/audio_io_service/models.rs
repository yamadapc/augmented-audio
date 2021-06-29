use cpal::traits::DeviceTrait;
use cpal::Device;
use serde::{Deserialize, Serialize};

pub type AudioHost = String;

#[derive(Serialize, Deserialize)]
pub struct AudioDevice {
    pub name: String,
}

impl AudioDevice {
    pub fn new(name: String) -> Self {
        AudioDevice { name }
    }

    pub fn from_device(device: Device) -> Result<Self, cpal::DeviceNameError> {
        Ok(Self::new(device.name()?))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DevicesList {
    pub input_devices: Vec<AudioDevice>,
    pub output_devices: Vec<AudioDevice>,
}

impl DevicesList {
    pub fn new(input_devices: Vec<AudioDevice>, output_devices: Vec<AudioDevice>) -> Self {
        DevicesList {
            input_devices,
            output_devices,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioIOState {
    pub host: AudioHost,
    pub input_device: Option<AudioDevice>,
    pub output_device: Option<AudioDevice>,
}
