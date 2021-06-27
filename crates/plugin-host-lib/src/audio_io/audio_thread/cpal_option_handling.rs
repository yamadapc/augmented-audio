use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Host};

use crate::audio_io::audio_thread::error::AudioThreadError;
use crate::audio_io::audio_thread::options::{AudioDeviceId, AudioHostId, AudioThreadOptions};

fn find_cpal_host_by_name(host_name: &str) -> Option<cpal::Host> {
    let maybe_id = cpal::available_hosts()
        .into_iter()
        .find(|host| host.name() == host_name);
    maybe_id
        .map(|host_id| cpal::host_from_id(host_id).ok())
        .flatten()
}

fn find_cpal_device_by_name(host: &Host, id: &str) -> Option<Device> {
    host.output_devices()
        .ok()
        .map(|mut devices| {
            devices.find(|device| {
                let name = device.name();
                match name {
                    Ok(name) => name == id,
                    Err(_) => false,
                }
            })
        })
        .flatten()
}

pub fn get_cpal_host(host_id: &AudioHostId) -> cpal::Host {
    match &host_id {
        AudioHostId::Default => cpal::default_host(),
        AudioHostId::Id(id) => find_cpal_host_by_name(&id).unwrap_or_else(cpal::default_host),
    }
}

pub fn get_cpal_output_device(
    host: &cpal::Host,
    output_device_id: &AudioDeviceId,
) -> Result<cpal::Device, AudioThreadError> {
    let maybe_device = match &output_device_id {
        AudioDeviceId::Default => host.default_output_device(),
        AudioDeviceId::Id(id) => find_cpal_device_by_name(host, id),
    };

    match maybe_device {
        Some(device) => Ok(device),
        None => Err(AudioThreadError::OutputDeviceNotFoundError),
    }
}

pub fn get_output_config(
    options: &AudioThreadOptions,
    output_device: &cpal::Device,
) -> Result<cpal::StreamConfig, AudioThreadError> {
    let output_config = output_device.default_output_config()?;
    let sample_format = output_config.sample_format();
    let mut output_config: cpal::StreamConfig = output_config.into();
    output_config.buffer_size = options.buffer_size.clone().into();

    if sample_format != cpal::SampleFormat::F32 {
        return Err(AudioThreadError::UnsupportedSampleFormat);
    }
    Ok(output_config)
}
