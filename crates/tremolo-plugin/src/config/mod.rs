use std::path::PathBuf;

pub mod logging;

pub fn get_configuration_root_path() -> PathBuf {
    let home_path = dirs::home_dir().unwrap_or(PathBuf::from(""));
    let root_path = home_path.join(".ruas");
    root_path
}
