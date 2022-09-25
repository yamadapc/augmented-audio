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

use anyhow::Error;
use chrono::prelude::*;
use cmd_lib::spawn;
use std::path::Path;

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

struct BuildAndPublishOptions<'a> {
    crate_path: &'a str,
    public_name: &'a str,
    public_path: &'a str,
    cargo_toml: &'a CargoToml,
    release_key: &'a str,
    example_name: Option<&'a str>,
    upload: bool,
}

impl BuildCommandService {
    pub fn run_build(&mut self, crate_path: Option<&str>, upload: bool) {
        log::info!("Starting build crate_path={:?}", crate_path);

        if let Some(crate_path) = crate_path {
            self.run_build_crate(crate_path, upload)
        } else {
            self.run_build_all(upload)
        }
    }

    fn run_build_all(&mut self, upload: bool) {
        let manifests = self.list_crates_service.find_manifests();
        for (manifest_path, manifest) in manifests {
            // Some packages shouldn't be built by default, those are marked with `package.metadata.skip = true`
            let metadata = &manifest.package.metadata;
            let skip = metadata.as_ref().and_then(|m| m.skip).unwrap_or(false);
            if skip {
                continue;
            }
            self.run_build_crate(&manifest_path, upload);
        }
    }

    fn run_build_crate(&mut self, crate_path: &str, upload: bool) {
        if self.run_build_crate_vst(crate_path, upload).is_none() {
            self.force_build(crate_path, &None).unwrap();
        }
    }

    fn run_build_crate_vst(&mut self, crate_path: &str, upload: bool) -> Option<()> {
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
            self.run_build_and_publish(BuildAndPublishOptions {
                crate_path,
                public_name: &public_name,
                public_path: &public_path,
                cargo_toml: &cargo_toml,
                release_key: &release_key,
                example_name: Some(&example),
                upload,
            })
        }

        if let Some(app_config) = &metadata.app {
            let public_name = &app_config.public_name;
            let public_path = self.get_public_path(&release_key, public_name);
            self.run_build_and_publish(BuildAndPublishOptions {
                crate_path,
                public_name,
                public_path: &public_path,
                cargo_toml: &cargo_toml,
                release_key: &release_key,
                example_name: None,
                upload,
            })
        }

        Some(())
    }

    fn run_build_and_publish(&mut self, options: BuildAndPublishOptions) {
        let BuildAndPublishOptions {
            crate_path,
            public_name,
            public_path,
            cargo_toml,
            release_key,
            example_name,
            upload,
        } = options;
        let release_json = ReleaseJson {
            name: cargo_toml.package.name.clone(),
            key: release_key.to_string(),
            created_at: Utc::now().to_rfc3339(),
            release_notes: None,
            file_download_url: public_path.to_string(),
            user_download_url: None,
        };

        log::debug!("Release: {:?}", release_json);
        log::debug!("Read Cargo.toml: {:?}", cargo_toml);

        // Force the package to be built
        self.force_build(crate_path, &example_name).unwrap();

        if let (Some(local_package), true) = (
            self.packager_service.create_local_package(PackagerInput {
                public_name: public_name.to_string(),
                crate_path: crate_path.to_string(),
                cargo_toml: cargo_toml.clone(),
                release_json: release_json.clone(),
                example_name: example_name.map(|s| s.to_string()),
            }),
            upload,
        ) {
            self.release_service.release(ReleaseInput {
                cargo_toml,
                local_package: &local_package,
                release_json: &release_json,
            })
        }
    }

    fn force_build(&self, crate_path: &str, example_name: &Option<&str>) -> anyhow::Result<()> {
        build_crate_universal_binary(&*self.cargo_toml_reader, crate_path, example_name)
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

#[derive(Debug)]
enum CrateArtifact {
    Bin,
    CDylib,
    Rlib,
}

fn build_crate_universal_binary(
    cargo_toml_reader: &dyn CargoTomlReader,
    crate_path: &str,
    example_name: &Option<&str>,
) -> Result<(), Error> {
    let current = std::env::current_dir()?;
    println!("- Building {} example={:?}", crate_path, example_name);
    log::debug!("cd {}", crate_path);

    let cargo_toml = cargo_toml_reader.read(crate_path);
    let artifacts = get_crate_artifacts(&std::path::PathBuf::from(crate_path), &cargo_toml);
    log::info!("Crate has artifacts: {:?}", artifacts);

    std::env::set_current_dir(crate_path)?;
    if let Some(example) = &example_name {
        println!("cargo build --release --example {} (multi-arch)", example);
        spawn!(cargo build --target aarch64-apple-darwin --release --example ${example})?.wait()?;
        spawn!(cargo build --target x86_64-apple-darwin --release --example ${example})?.wait()?;
    } else {
        // VST plugins are only recognized by Ableton if they are universal binaries, so we must
        // build both architectures and combine them with lipo
        println!("cargo build --release (multi-arch)");
        spawn!(cargo build --target aarch64-apple-darwin --release)?.wait()?;
        spawn!(cargo build --target x86_64-apple-darwin --release)?.wait()?;
    }
    std::env::set_current_dir(current)?;

    if cargo_toml.is_augmented_crate() {
        if let Some(lib_name) = cargo_toml.lib.and_then(|l| l.name) {
            let lipo_args = vec![
                format!(
                    "./target/aarch64-apple-darwin/release/lib{}.dylib",
                    lib_name
                ),
                format!("./target/x86_64-apple-darwin/release/lib{}.dylib", lib_name),
                "-output".to_string(),
                format!("./target/release/lib{}.dylib", lib_name),
            ];
            spawn!(lipo -create $[lipo_args])?.wait()?;
        } else if artifacts
            .iter()
            .any(|a| matches!(a, CrateArtifact::Bin { .. }))
        {
            let bin_name = &cargo_toml.package.name;
            log::info!("Creating universal binary for bin crate {}", bin_name);
            let lipo_args = vec![
                format!("./target/aarch64-apple-darwin/release/{}", bin_name),
                format!("./target/x86_64-apple-darwin/release/{}", bin_name),
                "-output".to_string(),
                format!("./target/release/{}", bin_name),
            ];
            spawn!(lipo -create $[lipo_args])?.wait()?;
        }

        if let Some(example) = &example_name {
            let lipo_args = vec![
                format!(
                    "./target/aarch64-apple-darwin/release/examples/lib{}.dylib",
                    example
                ),
                format!(
                    "./target/x86_64-apple-darwin/release/examples/lib{}.dylib",
                    example
                ),
                "-output".to_string(),
                format!("./target/release/examples/lib{}.dylib", example),
            ];
            spawn!(lipo -create $[lipo_args])?.wait()?;
        }
    }

    Ok(())
}

fn get_crate_artifacts(crate_path: &Path, manifest: &CargoToml) -> Vec<CrateArtifact> {
    let mut artifacts = vec![];

    let src_path = crate_path.join("src");
    if src_path.join("main.rs").exists() {
        artifacts.push(CrateArtifact::Bin);
    }
    if src_path.join("lib.rs").exists() {
        artifacts.push(CrateArtifact::Rlib);

        if manifest
            .lib
            .as_ref()
            .and_then(|l| l.crate_type.as_ref())
            .unwrap_or(&vec![])
            .iter()
            .any(|t| *t == "cdylib")
        {
            artifacts.push(CrateArtifact::CDylib);
        }
    }

    artifacts
}
