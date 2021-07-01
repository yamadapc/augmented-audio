use plugin_host_lib::audio_io::audio_io_service::storage::StorageConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct AppConfig {
  pub storage_config: StorageConfig,
}
