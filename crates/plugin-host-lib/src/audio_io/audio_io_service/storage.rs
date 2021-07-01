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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_store_file() {
        let tmp_dir = std::env::temp_dir();
        std::fs::create_dir_all(&tmp_dir).unwrap();
        let tmp_file_path = tmp_dir.join("plugin-host-lib-test-store-file.json");
        let _ = std::fs::remove_file(&tmp_file_path);

        let config = StorageConfig {
            audio_io_state_storage_path: String::from(tmp_file_path.to_str().unwrap()),
        };
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
}
