use std::sync::{Arc, Mutex};

use actix::prelude::*;
use cpal::traits::HostTrait;
use serde::Serialize;
use thiserror::Error;

pub use models::*;
use storage::{AudioIOStorageService, StorageConfig};

use crate::audio_io::audio_thread::options::{AudioDeviceId, AudioHostId};
use crate::TestPluginHost;

pub mod models;
pub mod storage;

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
    #[error("Failed to read configuration from disk")]
    StorageError,
    #[error("Failed to build device configuration model")]
    BuildAudioDeviceError,
}

pub type AudioIOServiceResult<T> = Result<T, AudioIOServiceError>;

pub struct AudioIOService {
    host: Arc<Mutex<TestPluginHost>>,
    state: AudioIOState,
    storage: Addr<AudioIOStorageService>,
}

impl AudioIOService {
    pub fn new(host: Arc<Mutex<TestPluginHost>>, storage_config: StorageConfig) -> Self {
        AudioIOService {
            host,
            storage: AudioIOStorageService::new(storage_config).start(),
            state: AudioIOState {
                host: Self::default_host(),
                input_device: Self::default_input_device(),
                output_device: Self::default_output_device(),
            },
        }
    }
}

impl AudioIOService {
    fn set_host_id(&mut self, host_id: String) -> Result<(), AudioIOServiceError> {
        log::info!("Setting audio host");
        let mut host = self.host.lock().unwrap();
        host.set_host_id(AudioHostId::Id(host_id.clone()))
            .map_err(|err| {
                log::error!("Failed to set host {}", err);
                AudioIOServiceError::AudioThreadError
            })?;
        self.state.host = host_id;
        Ok(())
    }

    fn set_input_device_id(&mut self, input_device_id: String) -> Result<(), AudioIOServiceError> {
        log::info!("Setting input device");
        let mut host = self.host.lock().unwrap();
        host.set_input_device_id(Some(AudioDeviceId::Id(input_device_id.clone())))
            .map_err(|err| {
                log::error!("Failed to set input device {}", err);
                AudioIOServiceError::AudioThreadError
            })?;

        let maybe_input_device = Self::input_devices(Some(self.state.host.clone()))
            .ok()
            .map(|input_devices| {
                input_devices
                    .into_iter()
                    .find(|input_device| input_device.name == input_device_id)
            })
            .flatten();
        if let Some(input_device) = maybe_input_device {
            self.state.input_device = Some(input_device);
        }

        Ok(())
    }

    pub fn set_output_device_id(
        &mut self,
        output_device_id: String,
    ) -> Result<(), AudioIOServiceError> {
        log::info!("Setting output device");
        let mut host = self.host.lock().unwrap();
        host.set_output_device_id(AudioDeviceId::Id(output_device_id.clone()))
            .map_err(|err| {
                log::error!("Failed to set output device {}", err);
                AudioIOServiceError::AudioThreadError
            })?;

        let maybe_output_device = Self::output_devices(Some(self.state.host.clone()))
            .ok()
            .map(|output_devices| {
                output_devices
                    .into_iter()
                    .find(|output_device| output_device.name == output_device_id)
            })
            .flatten();
        if let Some(output_device) = maybe_output_device {
            self.state.output_device = Some(output_device);
        }

        Ok(())
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
            .collect::<Result<Vec<AudioDevice>, BuildAudioDeviceError>>()
            .map_err(|_| AudioIOServiceError::BuildAudioDeviceError)?;
        Ok(devices_vec)
    }

    pub fn output_devices(host_id: Option<AudioHost>) -> AudioIOServiceResult<Vec<AudioDevice>> {
        let host = AudioIOService::host(&host_id)?;
        let devices = host
            .output_devices()
            .map_err(|_| AudioIOServiceError::DevicesError)?;
        let devices_vec = devices
            .map(AudioDevice::from_device)
            .collect::<Result<Vec<AudioDevice>, BuildAudioDeviceError>>()
            .map_err(|_| AudioIOServiceError::BuildAudioDeviceError)?;
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

impl Actor for AudioIOService {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<(), AudioIOServiceError>")]
pub struct ReloadMessage;

impl Handler<ReloadMessage> for AudioIOService {
    type Result = ResponseActFuture<Self, Result<(), AudioIOServiceError>>;

    fn handle(&mut self, _msg: ReloadMessage, _ctx: &mut Self::Context) -> Self::Result {
        let state = self.storage.send(storage::FetchMessage).into_actor(self);
        let result = state.map(|res, this, _ctx| match res {
            Ok(Ok(state)) => {
                log::info!("Reloaded state {:?}", state);
                this.state = state;

                let mut host = this.host.lock().unwrap();
                host.set_host_id(AudioHostId::Id(this.state.host.clone()))
                    .map_err(|_| AudioIOServiceError::AudioThreadError)?;
                host.set_input_device_id(Some(
                    this.state
                        .input_device
                        .clone()
                        .map(|d| AudioDeviceId::Id(d.name))
                        .unwrap_or(AudioDeviceId::Default),
                ))
                .map_err(|_| AudioIOServiceError::AudioThreadError)?;
                host.set_output_device_id(
                    this.state
                        .output_device
                        .clone()
                        .map(|d| AudioDeviceId::Id(d.name))
                        .unwrap_or(AudioDeviceId::Default),
                )
                .map_err(|_| AudioIOServiceError::AudioThreadError)?;

                Ok(())
            }
            _ => Err(AudioIOServiceError::StorageError),
        });

        Box::pin(result)
    }
}

#[derive(Message)]
#[rtype(result = "AudioIOState")]
pub struct GetStateMessage;

impl Handler<GetStateMessage> for AudioIOService {
    type Result = MessageResult<GetStateMessage>;

    fn handle(&mut self, _msg: GetStateMessage, _ctx: &mut Self::Context) -> Self::Result {
        MessageResult(self.state.clone())
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), AudioIOServiceError>")]
pub enum SetStateMessage {
    SetHostId { host_id: String },
    SetInputDeviceId { input_device_id: String },
    SetOutputDeviceId { output_device_id: String },
}

impl Handler<SetStateMessage> for AudioIOService {
    type Result = ResponseActFuture<Self, Result<(), AudioIOServiceError>>;

    fn handle(&mut self, msg: SetStateMessage, _ctx: &mut Self::Context) -> Self::Result {
        use SetStateMessage::*;

        let result = match msg {
            SetHostId { host_id } => self.set_host_id(host_id),
            SetInputDeviceId { input_device_id } => self.set_input_device_id(input_device_id),
            SetOutputDeviceId { output_device_id } => self.set_host_id(output_device_id),
        };

        let store_future = self.storage.send(storage::StoreMessage {
            state: self.state.clone(),
        });
        let result_future = store_future.into_actor(self).map(|storage_result, _, _| {
            if let Err(err) = storage_result {
                let err: &dyn std::error::Error = &err;
                log::error!("{}", err);
            }

            result
        });

        Box::pin(result_future)
    }
}

#[cfg(target_os = "macos")]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_io_service_get_hosts() {
        let hosts = AudioIOService::hosts();
        assert!(!hosts.is_empty());
    }

    #[test]
    fn test_io_service_get_inputs() {
        let host = None;
        let inputs = AudioIOService::input_devices(host).unwrap();
        assert!(!inputs.is_empty());
    }

    #[test]
    fn test_io_service_get_outputs() {
        let host = None;
        let outputs = AudioIOService::output_devices(host).unwrap();
        assert!(!outputs.is_empty());
    }
}
