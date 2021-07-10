use std::path::PathBuf;

pub mod logging;

pub fn get_configuration_root_path() -> PathBuf {
    let home_path = dirs::home_dir().unwrap_or_else(|| PathBuf::from(""));
    home_path.join(".ruas")
}

pub fn init(name: &str) {
    logging::configure_logging(&get_configuration_root_path(), name);
}
