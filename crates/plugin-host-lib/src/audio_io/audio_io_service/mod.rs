use cpal::traits::HostTrait;
use serde::Serialize;
use thiserror::Error;

pub use responses::*;

pub mod responses;

#[derive(Error, Debug, Serialize)]
pub enum AudioIOServiceError {
    #[error("Failed to get host reference")]
    HostUnavailableError,
    #[error("Failed to get host devices list")]
    DevicesError,
    #[error("Failed to get device name")]
    DeviceNameError,
}

pub type AudioIOServiceResult<T> = Result<T, AudioIOServiceError>;

pub struct AudioIOService {}

impl AudioIOService {
    pub fn hosts() -> Vec<String> {
        log::info!("Listing hosts");
        let hosts = cpal::available_hosts();
        hosts
            .into_iter()
            .map(|host| host.name().to_string())
            .collect()
    }

    pub fn devices_list(host_id: Option<String>) -> AudioIOServiceResult<DevicesList> {
        let inputs = Self::input_devices(host_id.clone())?;
        let outputs = Self::output_devices(host_id)?;
        Ok(DevicesList::new(inputs, outputs))
    }

    pub fn input_devices(host_id: Option<String>) -> AudioIOServiceResult<Vec<AudioDevice>> {
        let host = AudioIOService::host(&host_id)?;
        let devices = host
            .input_devices()
            .map_err(|_| AudioIOServiceError::DevicesError)?;
        let devices_vec = devices
            .map(AudioDevice::from_device)
            .collect::<Result<Vec<AudioDevice>, cpal::DeviceNameError>>()
            .map_err(|_| AudioIOServiceError::DeviceNameError)?;
        Ok(devices_vec)
    }

    pub fn output_devices(host_id: Option<String>) -> AudioIOServiceResult<Vec<AudioDevice>> {
        let host = AudioIOService::host(&host_id)?;
        let devices = host
            .output_devices()
            .map_err(|_| AudioIOServiceError::DevicesError)?;
        let devices_vec = devices
            .map(AudioDevice::from_device)
            .collect::<Result<Vec<AudioDevice>, cpal::DeviceNameError>>()
            .map_err(|_| AudioIOServiceError::DeviceNameError)?;
        Ok(devices_vec)
    }

    pub(crate) fn host(host_id: &Option<String>) -> AudioIOServiceResult<cpal::Host> {
        let host_id = host_id
            .as_ref()
            .map(|host_id| {
                cpal::available_hosts()
                    .into_iter()
                    .find(|host| host.name() == host_id)
            })
            .flatten()
            .unwrap_or_else(|| cpal::default_host().id());
        cpal::host_from_id(host_id).map_err(|_| AudioIOServiceError::HostUnavailableError)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_io_service_get_hosts() {
        let hosts = AudioIOService::hosts();
        assert!(hosts.len() > 0);
    }

    #[test]
    fn test_io_service_get_inputs() {
        let host = None;
        let inputs = AudioIOService::input_devices(host).unwrap();
        assert!(inputs.len() > 0);
    }

    #[test]
    fn test_io_service_get_outputs() {
        let host = None;
        let outputs = AudioIOService::output_devices(host).unwrap();
        assert!(outputs.len() > 0);
    }
}
