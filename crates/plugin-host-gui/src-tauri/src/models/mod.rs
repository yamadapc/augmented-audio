use serde::Serialize;

#[derive(Serialize)]
pub struct AudioConfiguration {
  pub host_id: String,
  pub input_id: String,
  pub output_id: String,
}

#[derive(Serialize)]
pub struct Project {
  pub id: String,
  pub title: String,
  pub audio_configuration: AudioConfiguration,
  pub input_file_path: String,
  pub plugin_path: String,
}
