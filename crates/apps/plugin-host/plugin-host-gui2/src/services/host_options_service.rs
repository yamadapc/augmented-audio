use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Clone, Default, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HostState {
    pub plugin_path: Option<String>,
    pub audio_input_file_path: Option<String>,
}

#[derive(Error, Debug)]
pub enum HostOptionsServiceError {
    #[error("Failed to serialize value")]
    SerdeError(#[from] serde_json::Error),
    #[error("IO error")]
    IOError(#[from] std::io::Error),
}

pub struct HostOptionsService {
    storage_path: String,
}

impl HostOptionsService {
    pub fn new(storage_path: String) -> Self {
        HostOptionsService { storage_path }
    }
}

impl HostOptionsService {
    pub fn fetch(&self) -> Result<HostState, HostOptionsServiceError> {
        let file = std::fs::File::open(&self.storage_path)?;
        Ok(serde_json::from_reader(file)?)
    }

    pub fn store(&self, state: &HostState) -> Result<(), HostOptionsServiceError> {
        let file = std::fs::File::create(&self.storage_path)?;
        Ok(serde_json::to_writer_pretty(file, state)?)
    }
}
