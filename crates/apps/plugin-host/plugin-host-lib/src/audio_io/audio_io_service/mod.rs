// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
use actix::prelude::*;
use cpal::traits::HostTrait;
use serde::Serialize;
use std::pin::Pin;
use thiserror::Error;

use crate::audio_io::audio_thread;
use crate::audio_io::audio_thread::AudioThread;
pub use models::*;
use storage::{AudioIOStorageService, StorageConfig};

use crate::audio_io::audio_thread::options::{AudioDeviceId, AudioHostId};

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
    #[error("Failed send messages")]
    MailboxError,
}

pub type AudioIOServiceResult<T> = Result<T, AudioIOServiceError>;

pub struct AudioIOService {
    audio_thread: Addr<AudioThread>,
    state: AudioIOState,
    storage: Addr<AudioIOStorageService>,
}

impl AudioIOService {
    pub fn new(audio_thread: Addr<AudioThread>, storage_config: StorageConfig) -> Self {
        AudioIOService {
            audio_thread,
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
    fn set_host_id(
        &mut self,
        host_id: String,
    ) -> impl ActorFuture<Self, Output = Result<(), AudioIOServiceError>> {
        log::info!("Setting audio host");
        self.audio_thread
            .send(audio_thread::actor::AudioThreadMessage::SetHost {
                host_id: AudioHostId::Id(host_id.clone()),
            })
            .into_actor(self)
            .map_err(|err, _, _| {
                log::error!("Failed to set host {}", err);
                AudioIOServiceError::AudioThreadError
            })
            .map(|_, this, _| {
                this.state.host = host_id;
                Ok(())
            })
    }

    fn set_input_device_id(
        &mut self,
        input_device_id: String,
    ) -> impl ActorFuture<Self, Output = Result<(), AudioIOServiceError>> {
        log::info!("Setting input device");

        self.audio_thread
            .send(audio_thread::actor::AudioThreadMessage::SetInputDevice {
                input_device_id: Some(AudioDeviceId::Id(input_device_id.clone())),
            })
            .into_actor(self)
            .map_err(|err, _, _| {
                log::error!("Failed to set input device {}", err);
                AudioIOServiceError::AudioThreadError
            })
            .map(move |_, this, _| {
                let maybe_input_device = Self::input_devices(Some(this.state.host.clone()))
                    .ok()
                    .and_then(|input_devices| {
                        input_devices
                            .into_iter()
                            .find(|input_device| input_device.name == input_device_id)
                    });
                if let Some(input_device) = maybe_input_device {
                    this.state.input_device = Some(input_device);
                }

                Ok(())
            })
    }

    pub fn set_output_device_id(
        &mut self,
        output_device_id: String,
    ) -> impl ActorFuture<Self, Output = Result<(), AudioIOServiceError>> {
        log::info!("Setting output device");
        self.audio_thread
            .send(audio_thread::actor::AudioThreadMessage::SetOutputDevice {
                output_device_id: AudioDeviceId::Id(output_device_id.clone()),
            })
            .into_actor(self)
            .map_err(|err, _, _| {
                log::error!("Failed to set output device {}", err);
                AudioIOServiceError::AudioThreadError
            })
            .map(move |_, this, _| {
                let maybe_output_device = Self::output_devices(Some(this.state.host.clone()))
                    .ok()
                    .and_then(|output_devices| {
                        output_devices
                            .into_iter()
                            .find(|output_device| output_device.name == output_device_id)
                    });

                if let Some(output_device) = maybe_output_device {
                    this.state.output_device = Some(output_device);
                }

                Ok(())
            })
    }

    pub fn default_input_device() -> Option<AudioDevice> {
        let host = cpal::default_host();
        let input_device = host.default_input_device();

        input_device.and_then(|d| AudioDevice::from_device(d).ok())
    }

    pub fn default_output_device() -> Option<AudioDevice> {
        let host = cpal::default_host();
        let input_device = host.default_output_device();

        input_device.and_then(|d| AudioDevice::from_device(d).ok())
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
            .and_then(|host_id| {
                cpal::available_hosts()
                    .into_iter()
                    .find(|host| host.name() == host_id)
            })
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
        let result = state.then(|res, this, _ctx| {
            let output: Pin<Box<dyn ActorFuture<Self, Output = Result<(), AudioIOServiceError>>>> =
                match res {
                    Ok(Ok(state)) => {
                        log::info!("Reloaded state {:?}", state);
                        this.state = state;

                        Box::pin(
                            this.audio_thread
                                .send(audio_thread::actor::AudioThreadMessage::SetOptions {
                                    host_id: AudioHostId::Id(this.state.host.clone()),
                                    input_device_id: Some(
                                        this.state
                                            .input_device
                                            .clone()
                                            .map(|d| AudioDeviceId::Id(d.name))
                                            .unwrap_or(AudioDeviceId::Default),
                                    ),
                                    output_device_id: this
                                        .state
                                        .output_device
                                        .clone()
                                        .map(|d| AudioDeviceId::Id(d.name))
                                        .unwrap_or(AudioDeviceId::Default),
                                })
                                .into_actor(this)
                                .map_err(|_err, _, _| AudioIOServiceError::AudioThreadError)
                                // Replace with result flattening https://github.com/rust-lang/rust/issues/70142
                                .map(|result, _, _| match result {
                                    Ok(Ok(r)) => Ok(r),
                                    // audio thread error
                                    Ok(Err(_err)) => Err(AudioIOServiceError::AudioThreadError),
                                    // mailbox error
                                    Err(err) => Err(err),
                                }),
                        )
                    }
                    _ => Box::pin(
                        futures_util::future::ready(Err(AudioIOServiceError::StorageError))
                            .into_actor(this),
                    ),
                };
            output
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

        type DynResult =
            Pin<Box<dyn ActorFuture<AudioIOService, Output = Result<(), AudioIOServiceError>>>>;

        let audio_thread_future: DynResult = match msg {
            SetHostId { host_id } => Box::pin(self.set_host_id(host_id)),
            SetInputDeviceId { input_device_id } => {
                Box::pin(self.set_input_device_id(input_device_id))
            }
            SetOutputDeviceId { output_device_id } => {
                Box::pin(self.set_output_device_id(output_device_id))
            }
        };

        let result_future = audio_thread_future.then(|result, this, _| {
            let result_future: DynResult = if result.is_err() {
                Box::pin(futures_util::future::ready(result).into_actor(this))
            } else {
                let storage_future = this
                    .storage
                    .send(storage::StoreMessage {
                        state: this.state.clone(),
                    })
                    .into_actor(this)
                    .map_err(|_err, _, _| AudioIOServiceError::AudioThreadError)
                    .map(move |storage_result, _, _| {
                        match storage_result {
                            // Ignore storage errors
                            Ok(Ok(_)) => Ok(()),
                            Ok(Err(err)) => {
                                let err: &dyn std::error::Error = &err;
                                log::error!("{}", err);
                                Ok(())
                            }
                            _ => {
                                log::error!("Mailbox error storing audio settings");
                                Ok(())
                            }
                        }
                    });
                Box::pin(storage_future)
            };
            result_future
        });

        Box::pin(result_future)
    }
}

// These tests are skipped because they don't make sense in CI
//
// #[cfg(target_os = "macos")]
// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_io_service_get_hosts() {
//         let hosts = AudioIOService::hosts();
//         assert!(!hosts.is_empty());
//     }
//     #[test]
//     fn test_io_service_get_inputs() {
//         let host = None;
//         let inputs = AudioIOService::input_devices(host).unwrap();
//         assert!(!inputs.is_empty());
//     }
//     #[test]
//     fn test_io_service_get_outputs() {
//         let host = None;
//         let outputs = AudioIOService::output_devices(host).unwrap();
//         assert!(!outputs.is_empty());
//     }
// }
