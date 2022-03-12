use std::path::{Path, PathBuf};

use mockall::automock;

use app_template_handler::AppTemplateHandler;
use vst_handler::VstHandler;

use crate::manifests::{CargoToml, MacosAppConfig, ReleaseJson, VstConfig};

mod app_template_handler;
mod vst_handler;

/// Represents an App package that has been built
pub struct PackagerInput<'a> {
    pub public_name: &'a str,
    pub crate_path: &'a str,
    pub cargo_toml: &'a CargoToml,
    pub release_json: &'a ReleaseJson,
    pub example_name: Option<&'a str>,
}

/// Represents an App package that has been built
pub struct LocalPackage {
    pub path: String,
    pub target_app_path: PathBuf,
}

#[automock]
pub trait PackagerService {
    #[allow(clippy::needless_lifetimes)]
    fn create_local_package<'a>(&self, input: PackagerInput<'a>) -> Option<LocalPackage>;
}

#[derive(Default)]
pub struct PackagerServiceImpl {}

impl PackagerService for PackagerServiceImpl {
    fn create_local_package(&self, input: PackagerInput) -> Option<LocalPackage> {
        let macos_config = input
            .cargo_toml
            .package
            .metadata
            .clone().and_then(|m| m.app).and_then(|a| a.macos);

        if let Some(example) = input.example_name {
            let target_path =
                Self::build_target_path(&input.cargo_toml.package.name, &input.release_json.key);
            return VstHandler::handle(
                target_path,
                &input,
                VstConfig {
                    identifier: format!(
                        "com.beijaflor.{}__{}",
                        input.cargo_toml.package.name.replace('-', "_"),
                        example.replace('-', "_")
                    ),
                },
            );
        }

        if let Some(macos_config) = macos_config {
            let target_path =
                Self::build_target_path(&input.cargo_toml.package.name, &input.release_json.key);
            let result = match macos_config {
                MacosAppConfig::AppTemplate(config) => {
                    AppTemplateHandler::handle(target_path, &input, config)
                }
                MacosAppConfig::Vst(vst) => VstHandler::handle(target_path, &input, vst),
            };

            log::info!("Updating latest symlink");
            let package_name = &input.cargo_toml.package.name;
            let release_key = &input.release_json.key;
            let release_path = Path::new(release_key);
            let latest_path = Path::new("./target/apps/macos/")
                .join(package_name)
                .join("release-latest");
            cmd_lib::run_cmd!(rm -f $latest_path).unwrap();
            log::info!(
                "ln -s {} {}",
                release_path.to_str().unwrap(),
                latest_path.to_str().unwrap()
            );
            cmd_lib::run_cmd!(ln -s $release_path/ $latest_path).unwrap();

            result
        } else {
            log::error!("There's no package config");
            None
        }
    }
}

impl PackagerServiceImpl {
    fn build_target_path(package_name: &str, release_key: &str) -> PathBuf {
        let base_target_path = Path::new("./target/apps/macos/")
            .join(package_name)
            .join(release_key)
            .join("artifacts");
        base_target_path
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_build_target_path() {
        let path = PackagerServiceImpl::build_target_path("looper-vst", "release-0.0.1-abcdef");
        assert_eq!(
            path.to_str().unwrap(),
            "./target/apps/macos/looper-vst/release-0.0.1-abcdef/artifacts"
        );
    }
}
