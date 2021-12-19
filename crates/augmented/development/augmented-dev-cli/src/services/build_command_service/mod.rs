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

mod git_release_provider;
mod packager_service;
mod release_service;

pub struct BuildCommandService {
    cargo_toml_reader: Box<dyn CargoTomlReader>,
    git_release_provider: Box<dyn GitReleaseProvider>,
    packager_service: Box<dyn PackagerService>,
    release_service: Box<dyn ReleaseService>,
}

impl Default for BuildCommandService {
    fn default() -> Self {
        Self {
            cargo_toml_reader: Box::new(CargoTomlReaderImpl::default()),
            git_release_provider: Box::new(GitReleaseProviderImpl::default()),
            packager_service: Box::new(PackagerServiceImpl::default()),
            release_service: Box::new(ReleaseServiceImpl::default()),
        }
    }
}

impl BuildCommandService {
    pub fn run_build(&mut self, crate_path: &str, example_name: Option<&str>) {
        log::info!(
            "Starting build crate={} example={:?}",
            crate_path,
            example_name
        );

        let cargo_toml = self.cargo_toml_reader.read(crate_path);
        let release_key = self
            .git_release_provider
            .get_key(&cargo_toml.package.version);

        let metadata = cargo_toml
            .package
            .metadata
            .as_ref()
            .expect("No package.metadata section found");
        let public_name = example_name.unwrap_or_else(|| {
            let app = metadata
                .app
                .as_ref()
                .expect("No package.metadata.app section found");
            &app.public_name
        });
        let public_path = self.get_public_path(&release_key, public_name);

        let release_json = ReleaseJson {
            name: cargo_toml.package.name.clone(),
            key: release_key,
            created_at: Utc::now().to_rfc3339(),
            release_notes: None,
            file_download_url: public_path,
            user_download_url: None,
        };

        log::info!("Release:\n{:#?}", release_json);
        log::info!("Read Cargo.toml:\n{:#?}", cargo_toml);

        // Force the package to be built
        BuildCommandService::force_build(crate_path, &cargo_toml, &example_name);

        if let Some(local_package) = self.packager_service.create_local_package(PackagerInput {
            crate_path,
            cargo_toml: &cargo_toml,
            release_json: &release_json,
            example_name,
        }) {
            self.release_service.release(ReleaseInput {
                cargo_toml: &cargo_toml,
                local_package: &local_package,
                release_json: &release_json,
            })
        }
    }

    fn force_build(crate_path: &str, cargo_toml: &CargoToml, example_name: &Option<&str>) {
        log::info!("Forcing a build of \"{}\"", cargo_toml.package.name);
        {
            let current = std::env::current_dir().unwrap();
            std::env::set_current_dir(crate_path).unwrap();
            if let Some(example) = example_name {
                spawn!(cargo build --release --example ${example})
                    .unwrap()
                    .wait()
                    .unwrap();
            } else {
                spawn!(cargo build --release).unwrap().wait().unwrap();
            }
            std::env::set_current_dir(current).unwrap();
        }
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
