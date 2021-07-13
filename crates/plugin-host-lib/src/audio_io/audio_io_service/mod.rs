use std::sync::{Arc, Mutex};

use cpal::traits::HostTrait;
use serde::Serialize;
use thiserror::Error;

pub use models::*;
use storage::{AudioIOStorageService, StorageConfig};

use crate::audio_io::audio_io_service::storage::AudioIOStorageServiceError;
use crate::audio_io::audio_thread::options::{AudioDeviceId, AudioHostId};
use crate::TestPluginHost;

pub mod models;
pub mod storage;

fn log_error<T, Err: std::error::Error>(r: Result<T, Err>) {
    if let Err(err) = r {
        log::error!("{}", err);
    }
}

#[derive(Error, Debug, Serialize)]
pub enum AudioIOServiceError {
    #[error("Failed to get host reference")]
    HostUnavailableError,
    #[error("Failed to get host devices list")]
    DevicesError,
    #[error("Failed to get device name")]
    DeviceNameError,
    #[error("Failed to perform audio thread changes")]
    AudioThreadError,
}

pub type AudioIOServiceResult<T> = Result<T, AudioIOServiceError>;

pub struct AudioIOService {
    host: Arc<Mutex<TestPluginHost>>,
    state: AudioIOState,
    storage: AudioIOStorageService,
}

impl AudioIOService {
    pub fn new(host: Arc<Mutex<TestPluginHost>>, storage_config: StorageConfig) -> Self {
        AudioIOService {
            host,
            storage: AudioIOStorageService::new(storage_config),
            state: AudioIOState {
                host: Self::default_host(),
                input_device: Self::default_input_device(),
                output_device: Self::default_output_device(),
            },
        }
    }
}

impl AudioIOService {
    pub fn store(&self) -> Result<(), AudioIOStorageServiceError> {
        self.storage.store(self.state())
    }

    pub fn try_store(&self) {
        log_error(self.store());
    }

    pub fn reload(&mut self) -> Result<(), AudioIOStorageServiceError> {
        let state = self.storage.fetch()?;
        self.state = state;
        Ok(())
    }

    pub fn state(&self) -> &AudioIOState {
        &self.state
    }

    pub fn set_host_id(&mut self, host_id: String) -> Result<(), AudioIOServiceError> {
        let mut host = self.host.lock().unwrap();
        host.set_host_id(AudioHostId::Id(host_id.clone()))
            .map_err(|err| {
                log::error!("Failed to set host {}", err);
                AudioIOServiceError::AudioThreadError
            })?;
        self.state.host = host_id;
        self.try_store();
        Ok(())
    }

    pub fn set_input_device_id(
        &mut self,
        input_device_id: String,
    ) -> Result<(), AudioIOServiceError> {
        let mut host = self.host.lock().unwrap();
        host.set_output_device_id(AudioDeviceId::Id(input_device_id.clone()))
            .map_err(|err| {
                log::error!("Failed to set input device {}", err);
                AudioIOServiceError::AudioThreadError
            })?;
        self.state.input_device = Some(AudioDevice::new(input_device_id));
        self.try_store();
        Ok(())
    }

    pub fn set_output_device_id(
        &mut self,
        output_device_id: String,
    ) -> Result<(), AudioIOServiceError> {
        let mut host = self.host.lock().unwrap();
        let result = host
            .set_output_device_id(AudioDeviceId::Id(output_device_id.clone()))
            .map_err(|err| {
                log::error!("Failed to set output device {}", err);
                AudioIOServiceError::AudioThreadError
            })?;
        self.state.input_device = Some(AudioDevice::new(output_device_id));
        self.try_store();
        Ok(result)
    }

    pub fn default_input_device() -> Option<AudioDevice> {
        let host = cpal::default_host();
        let input_device = host.default_input_device();

        input_device
            .map(|d| AudioDevice::from_device(d).ok())
            .flatten()
    }

    pub fn default_output_device() -> Option<AudioDevice> {
        let host = cpal::default_host();
        let input_device = host.default_output_device();

        input_device
            .map(|d| AudioDevice::from_device(d).ok())
            .flatten()
    }

    pub fn default_host() -> AudioHost {
        let host = cpal::default_host();
        host.id().name().to_string()
    }

    pub fn hosts() -> Vec<AudioHost> {
        log::info!("Listing hosts");
        let hosts = cpal::available_hosts();
        hosts
            .into_iter()
            .map(|host| host.name().to_string())
            .collect()
    }

    pub fn devices_list(host_id: Option<AudioHost>) -> AudioIOServiceResult<DevicesList> {
        let inputs = Self::input_devices(host_id.clone())?;
        let outputs = Self::output_devices(host_id)?;
        Ok(DevicesList::new(inputs, outputs))
    }

    pub fn input_devices(host_id: Option<AudioHost>) -> AudioIOServiceResult<Vec<AudioDevice>> {
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

    pub fn output_devices(host_id: Option<AudioHost>) -> AudioIOServiceResult<Vec<AudioDevice>> {
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

    pub fn host(host_id: &Option<AudioHost>) -> AudioIOServiceResult<cpal::Host> {
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

#[cfg(target_os = "macos")]
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
