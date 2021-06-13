use cpal::traits::DeviceTrait;
use cpal::Device;
use serde::Serialize;

#[derive(Serialize)]
pub struct AudioDevice {
    name: String,
}

impl AudioDevice {
    pub fn new(name: String) -> Self {
        AudioDevice { name }
    }

    pub fn from_device(device: Device) -> Result<Self, cpal::DeviceNameError> {
        Ok(Self::new(device.name()?))
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DevicesList {
    input_devices: Vec<AudioDevice>,
    output_devices: Vec<AudioDevice>,
}

impl DevicesList {
    pub fn new(input_devices: Vec<AudioDevice>, output_devices: Vec<AudioDevice>) -> Self {
        DevicesList {
            input_devices,
            output_devices,
        }
    }
}
