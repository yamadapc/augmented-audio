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
use std::path::{Path, PathBuf};

use mockall::automock;

use app_template_handler::AppTemplateHandler;
use vst_handler::VstHandler;

use crate::manifests::{CargoToml, MacosAppConfig, ReleaseJson, VstConfig};

mod app_template_handler;
mod vst_handler;

/// Represents an App package that has been built
#[derive(Debug, Clone)]
pub struct PackagerInput {
    pub public_name: String,
    pub crate_path: String,
    pub cargo_toml: CargoToml,
    pub release_json: ReleaseJson,
    pub example_name: Option<String>,
}

/// Represents an App package that has been built
#[derive(Debug)]
pub struct LocalPackage {
    pub path: String,
    pub target_app_path: PathBuf,
    pub input: PackagerInput,
}

#[automock]
pub trait PackagerService {
    #[allow(clippy::needless_lifetimes)]
    fn create_local_package(&self, input: PackagerInput) -> Option<LocalPackage>;
}

#[derive(Default)]
pub struct PackagerServiceImpl {}

impl PackagerService for PackagerServiceImpl {
    fn create_local_package(&self, input: PackagerInput) -> Option<LocalPackage> {
        let macos_config = input
            .cargo_toml
            .package
            .metadata
            .clone()
            .and_then(|m| m.app)
            .and_then(|a| a.macos);

        if let Some(example) = &input.example_name {
            let target_path =
                Self::build_target_path(&input.cargo_toml.package.name, &input.release_json.key);
            return VstHandler::handle(
                target_path,
                input.clone(),
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
                    AppTemplateHandler::handle(target_path, input.clone(), config)
                }
                MacosAppConfig::Vst(vst) => VstHandler::handle(target_path, input.clone(), vst),
            };
            log::info!("Finished packaging with result: {:#?}", result);

            log::info!("Updating latest symlink");
            let package_name = &input.cargo_toml.package.name;
            let release_key = &input.release_json.key;
            let release_path = Path::new(release_key);
            let latest_path = Path::new("./target/apps/macos/")
                .join(package_name)
                .join("release-latest");
            cmd_lib::run_cmd!(rm -rf $latest_path).unwrap();
            log::info!(
                "ln -s {} {}",
                release_path.to_str().unwrap(),
                latest_path.to_str().unwrap()
            );
            cmd_lib::run_cmd!(ln -s $release_path/ $latest_path).unwrap();

            if let Some(LocalPackage {
                target_app_path, ..
            }) = &result
            {
                cmd_lib::run_cmd!(mkdir -p ./target/apps/macos/latest/).unwrap();
                let target_app_path_base = Path::new(target_app_path).file_name().unwrap();
                let all_latest_path =
                    Path::new("./target/apps/macos/latest/").join(target_app_path_base);
                cmd_lib::run_cmd!(rm -rf $all_latest_path).unwrap();
                cmd_lib::run_cmd!(cp -r $target_app_path $all_latest_path).unwrap();
            }

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
