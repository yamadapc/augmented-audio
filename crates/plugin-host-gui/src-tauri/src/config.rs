use serde::{Deserialize, Serialize};

use plugin_host_lib::audio_io::audio_io_service::storage::StorageConfig;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct AppConfig {
  pub audio_thread_config_path: String,
  pub storage_config: StorageConfig,
}
