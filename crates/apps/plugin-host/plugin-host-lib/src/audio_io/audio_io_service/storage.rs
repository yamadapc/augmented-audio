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
use actix::{Actor, Context, Handler, Message};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::audio_io::AudioIOState;

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct StorageConfig {
    pub audio_io_state_storage_path: String,
}

#[derive(Error, Debug)]
pub enum AudioIOStorageServiceError {
    #[error("Failed to serialize value")]
    SerdeError(#[from] serde_json::Error),
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("Mailbox error")]
    MailboxError(#[from] actix::MailboxError),
}

pub struct AudioIOStorageService {
    config: StorageConfig,
}

impl AudioIOStorageService {
    pub fn new(config: StorageConfig) -> Self {
        AudioIOStorageService { config }
    }
}

impl AudioIOStorageService {
    pub fn fetch(&self) -> Result<AudioIOState, AudioIOStorageServiceError> {
        let file = std::fs::File::open(&self.config.audio_io_state_storage_path)?;
        Ok(serde_json::from_reader(file)?)
    }

    pub fn store(&self, state: &AudioIOState) -> Result<(), AudioIOStorageServiceError> {
        let file = std::fs::File::create(&self.config.audio_io_state_storage_path)?;
        Ok(serde_json::to_writer_pretty(file, state)?)
    }
}

impl Actor for AudioIOStorageService {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "Result<AudioIOState, AudioIOStorageServiceError>")]
pub struct FetchMessage;

#[derive(Message)]
#[rtype(result = "Result<(), AudioIOStorageServiceError>")]
pub struct StoreMessage {
    pub state: AudioIOState,
}

impl Handler<FetchMessage> for AudioIOStorageService {
    type Result = Result<AudioIOState, AudioIOStorageServiceError>;

    fn handle(&mut self, _msg: FetchMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.fetch()
    }
}

impl Handler<StoreMessage> for AudioIOStorageService {
    type Result = Result<(), AudioIOStorageServiceError>;

    fn handle(&mut self, msg: StoreMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.store(&msg.state)
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_store_file() {
        let (tmp_file_path, config) = build_tmp_config();
        let service = AudioIOStorageService::new(config);

        let state = AudioIOState {
            host: "Some host".to_string(),
            input_device: None,
            output_device: None,
        };
        service.store(&state).unwrap();
        let stored_state = service.fetch().unwrap();
        assert_eq!(stored_state, state);

        std::fs::remove_file(&tmp_file_path).unwrap();
    }

    #[actix::test]
    async fn test_actix_spawn() {
        let (tmp_file_path, config) = build_tmp_config();
        let service = AudioIOStorageService::new(config);
        let service = service.start();
        let state = AudioIOState {
            host: "Some host".to_string(),
            input_device: None,
            output_device: None,
        };
        service
            .send(StoreMessage {
                state: state.clone(),
            })
            .await
            .unwrap()
            .unwrap();
        let stored_state = service.send(FetchMessage).await.unwrap().unwrap();
        assert_eq!(stored_state, state);

        std::fs::remove_file(&tmp_file_path).unwrap();
    }

    fn build_tmp_config() -> (PathBuf, StorageConfig) {
        let id = uuid::Uuid::new_v4().to_string();
        let tmp_dir = std::env::temp_dir();
        std::fs::create_dir_all(&tmp_dir).unwrap();
        let tmp_file_path = tmp_dir.join(format!("plugin-host-lib-test-store-file__{}.json", id));
        let _ = std::fs::remove_file(&tmp_file_path);

        let config = StorageConfig {
            audio_io_state_storage_path: String::from(tmp_file_path.to_str().unwrap()),
        };
        (tmp_file_path, config)
    }
}
