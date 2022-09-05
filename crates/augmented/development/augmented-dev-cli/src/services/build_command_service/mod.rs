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
use chrono::prelude::*;
use cmd_lib::spawn;

use crate::manifests::{CargoToml, ReleaseJson};
use crate::services::build_command_service::git_release_provider::{
    GitReleaseProvider, GitReleaseProviderImpl,
};
use crate::services::build_command_service::packager_service::{
    PackagerInput, PackagerService, PackagerServiceImpl,
};
use crate::services::build_command_service::release_service::{
    ReleaseInput, ReleaseService, ReleaseServiceImpl,
};
use crate::services::cargo_toml_reader::{CargoTomlReader, CargoTomlReaderImpl};
use crate::services::ListCratesService;

mod git_release_provider;
mod packager_service;
mod release_service;

pub struct BuildCommandService {
    cargo_toml_reader: Box<dyn CargoTomlReader>,
    git_release_provider: Box<dyn GitReleaseProvider>,
    packager_service: Box<dyn PackagerService>,
    release_service: Box<dyn ReleaseService>,
    list_crates_service: Box<ListCratesService>,
}

impl Default for BuildCommandService {
    fn default() -> Self {
        Self {
            cargo_toml_reader: Box::new(CargoTomlReaderImpl::default()),
            git_release_provider: Box::new(GitReleaseProviderImpl::default()),
            packager_service: Box::new(PackagerServiceImpl::default()),
            release_service: Box::new(ReleaseServiceImpl::default()),
            list_crates_service: Box::new(ListCratesService::default()),
        }
    }
}

impl BuildCommandService {
    pub fn run_build(&mut self, crate_path: Option<&str>) {
        log::info!("Starting build crate={:?}", crate_path);

        if let Some(crate_path) = crate_path {
            self.run_build_crate(crate_path)
        } else {
            self.run_build_all()
        }
    }

    fn run_build_all(&mut self) {
        let manifests = self.list_crates_service.find_manifests();
        for (manifest_path, manifest) in manifests {
            // Some packages shouldn't be built by default, those are marked with `package.metadata.skip = true`
            let metadata = &manifest.package.metadata;
            let skip = metadata.as_ref().and_then(|m| m.skip).unwrap_or(false);
            if skip {
                continue;
            }
            self.run_build_crate(&manifest_path);
        }
    }

    fn run_build_crate(&mut self, crate_path: &str) {
        if self.run_build_crate_vst(crate_path).is_none() {
            BuildCommandService::force_build(crate_path, &None);
        }
    }

    fn run_build_crate_vst(&mut self, crate_path: &str) -> Option<()> {
        let cargo_toml = self.cargo_toml_reader.read(crate_path);
        let release_key = self
            .git_release_provider
            .get_key(&cargo_toml.package.version);

        let metadata = cargo_toml.package.metadata.as_ref()?;
        let vst_examples = metadata
            .augmented
            .as_ref()
            .and_then(|a| a.vst_examples.clone())
            .unwrap_or_default();

        for example in vst_examples {
            let package_name = &cargo_toml.package.name;
            let public_name = format!("{}__{}", package_name, example);
            let public_path = self.get_public_path(&release_key, &public_name);
            self.run_build_and_publish(
                crate_path,
                &public_name,
                &public_path,
                &cargo_toml,
                &release_key,
                Some(&example),
            )
        }

        if let Some(app_config) = &metadata.app {
            let public_name = &app_config.public_name;
            let public_path = self.get_public_path(&release_key, public_name);
            self.run_build_and_publish(
                crate_path,
                public_name,
                &public_path,
                &cargo_toml,
                &release_key,
                None,
            )
        }

        Some(())
    }

    fn run_build_and_publish(
        &mut self,
        crate_path: &str,
        public_name: &str,
        public_path: &str,
        cargo_toml: &CargoToml,
        release_key: &str,
        example_name: Option<&str>,
    ) {
        let release_json = ReleaseJson {
            name: cargo_toml.package.name.clone(),
            key: release_key.to_string(),
            created_at: Utc::now().to_rfc3339(),
            release_notes: None,
            file_download_url: public_path.to_string(),
            user_download_url: None,
        };

        log::info!("Release:\n{:#?}", release_json);
        log::info!("Read Cargo.toml:\n{:#?}", cargo_toml);

        // Force the package to be built
        BuildCommandService::force_build(crate_path, &example_name);

        if let Some(local_package) = self.packager_service.create_local_package(PackagerInput {
            public_name,
            crate_path,
            cargo_toml,
            release_json: &release_json,
            example_name,
        }) {
            self.release_service.release(ReleaseInput {
                cargo_toml,
                local_package: &local_package,
                release_json: &release_json,
            })
        }
    }

    fn force_build(crate_path: &str, example_name: &Option<&str>) {
        let current = std::env::current_dir().unwrap();
        log::info!("cd {}", crate_path);
        std::env::set_current_dir(crate_path).unwrap();
        if let Some(example) = example_name {
            log::info!("cargo build --release --example {}", example);
            spawn!(cargo build --release --example ${example})
                .unwrap()
                .wait()
                .unwrap();
        } else {
            log::info!("cargo build --release");
            spawn!(cargo build --release).unwrap().wait().unwrap();
        }
        std::env::set_current_dir(current).unwrap();
    }

    fn get_public_path(&self, release_key: &str, public_name: &str) -> String {
        let public_path = std::env::var("PUBLIC_PATH").unwrap();
        let dmg_name = format!("{}.dmg", public_name);
        let artifact_path = format!(
            "{}{}/{}",
            public_path,
            release_key,
            urlencoding::encode(&dmg_name)
        );
        artifact_path
    }
}
