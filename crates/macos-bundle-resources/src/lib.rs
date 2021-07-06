#[cfg(not(target_os = "macos"))]
pub use fallback::*;
#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(not(target_os = "macos"))]
mod fallback {
    use std::path::PathBuf;

    pub fn has_bundle(_bundle_identifier: &str) -> bool {
        false
    }

    pub fn get_path(
        _bundle_identifier: &str,
        _resource_name: &str,
        _resource_type: Option<&str>,
        _sub_dir_name: Option<&str>,
    ) -> Option<PathBuf> {
        None
    }
}
