use chrono::prelude::*;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use cmd_lib::{run_cmd, spawn};

use crate::manifests::{MacosAppConfig, ReleaseJson};
use manifests::CargoToml;

mod manifests;

fn get_cli_version() -> String {
    format!(
        "{}-{}-{}",
        env!("PROFILE"),
        env!("CARGO_PKG_VERSION"),
        env!("GIT_REV_SHORT")
    )
}

fn get_release_key(pkg_version: &str) -> String {
    let output = std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .unwrap();
    let git_rev = String::from_utf8(output.stdout).unwrap().trim().to_string();
    format!("release-{}-{}", pkg_version, git_rev)
}

fn run_build_release(crate_path: &str) {
    let config_path = Path::new(crate_path).join("Cargo.toml");
    let input_cargo_file = read_to_string(config_path).expect("Failed to read toml file");
    let cargo_toml: CargoToml =
        toml::from_str(&input_cargo_file).expect("Failed to parse toml file");
    let release_key = get_release_key(&cargo_toml.package.version);
    let target_path = get_target_path(&cargo_toml.package.name, &release_key);
    let upload_path = get_upload_path(&release_key);
    let public_path = get_public_path(
        &release_key,
        &cargo_toml
            .package
            .metadata
            .as_ref()
            .unwrap()
            .app
            .as_ref()
            .unwrap()
            .public_name,
    );

    let release_json = ReleaseJson {
        name: cargo_toml.package.name.clone(),
        key: release_key,
        created_at: Utc::now().to_rfc3339(),
        release_notes: None,
        file_download_url: public_path,
        user_download_url: None,
    };
    log::info!("Release:\n{:#?}", release_json);

    // Force the package to be built
    log::info!("Read Cargo.toml:\n{:#?}", cargo_toml);
    log::info!("Forcing a build of \"{}\"", cargo_toml.package.name);
    {
        let current = std::env::current_dir().unwrap();
        std::env::set_current_dir(crate_path);
        spawn!(cargo build --release)
            .unwrap()
            .wait_cmd_result()
            .unwrap();
        std::env::set_current_dir(current);
    }

    if let Some(local_package) = create_local_package(crate_path, &cargo_toml, &release_json) {
        sign_app(&local_package);
        let artifact_path = compress_release(&cargo_toml, local_package);
        upload_release(&release_json, artifact_path);
    }
}

fn sign_app(local_package: &LocalPackage) {
    log::info!("Code-signing the .app package");
    let certificate = std::env::var("SIGN_CERTIFICATE").unwrap();
    let app_path = &local_package.target_app_path;
    run_cmd!(codesign --force --sign "${certificate}" ${app_path}).unwrap();
}

fn upload_release(release_json: &ReleaseJson, artifact_path: String) {
    let bucket_path = get_upload_path(&release_json.key);
    log::info!(
        "Uploading to S3 release={} path={} bucket_path={}",
        release_json.key,
        artifact_path,
        bucket_path
    );
    run_cmd!(aws s3 cp --recursive ${artifact_path} ${bucket_path}).unwrap()
}

fn get_upload_path(release_key: &str) -> String {
    let bucket = std::env::var("AWS_S3_BUCKET").unwrap();
    let bucket_path = format!("{}{}", bucket, release_key);
    bucket_path
}

fn get_public_path(release_key: &str, public_name: &str) -> String {
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

fn compress_release(cargo_toml: &CargoToml, local_package: LocalPackage) -> String {
    let path = local_package.path;
    let volume_name = cargo_toml
        .package
        .metadata
        .as_ref()
        .unwrap()
        .app
        .as_ref()
        .unwrap()
        .public_name
        .clone();
    let dmg_name = format!("{}.dmg", volume_name);
    let target_path = {
        let path = Path::new(&*path);
        let parent = path.parent().unwrap();
        let dmg_path = parent.join(&*dmg_name);
        dmg_path.to_str().unwrap().to_string()
    };

    log::info!(
        "Creating DMG file VOLUME_NAME={} SOURCE={} TARGET={}",
        volume_name,
        path,
        target_path
    );
    run_cmd!(
        hdiutil create -volname ${volume_name} -srcfolder ${path} -ov -format UDZO ${target_path}
    )
    .unwrap();

    Path::new(&*path)
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

struct LocalPackage {
    path: String,
    target_app_path: PathBuf,
}

fn create_local_package(
    crate_path: &str,
    cargo_toml: &CargoToml,
    release_json: &ReleaseJson,
) -> Option<LocalPackage> {
    let macos_config = cargo_toml
        .package
        .metadata
        .clone()
        .map(|m| m.app)
        .flatten()
        .map(|a| a.macos)
        .flatten();
    if let Some(macos_config) = macos_config {
        match macos_config {
            MacosAppConfig::AppTemplate { template_path } => {
                let template_path = Path::new(crate_path).join(template_path);
                let base_target_path = get_target_path(&cargo_toml.package.name, &release_json.key);

                log::info!(
                    "Copying template into `{}` directory",
                    base_target_path.to_str().unwrap()
                );
                run_cmd!(mkdir -p ${base_target_path}).unwrap();
                run_cmd!(cp -r ${template_path} ${base_target_path}).unwrap();

                let release_path =
                    Path::new("./target/release/").join(cargo_toml.package.name.clone());
                let target_app_path = base_target_path.join(template_path.file_name().unwrap());
                run_cmd!(cp ${release_path} ${target_app_path}/Contents/MacOS/).unwrap();

                let release_json_path = base_target_path.join("release.json");
                log::info!("Outputting {}", release_json_path.to_str().unwrap());
                let release_json_str = serde_json::to_string_pretty(&release_json).unwrap();
                std::fs::write(release_json_path, release_json_str).unwrap();

                Some(LocalPackage {
                    path: base_target_path.to_str().unwrap().to_string(),
                    target_app_path,
                })
            }
            MacosAppConfig::Vst => {
                log::info!("VST build is unsupported");
                None
            }
        }
    } else {
        None
    }
}

fn get_target_path(package_name: &str, release_key: &str) -> PathBuf {
    let base_target_path = Path::new("./target/apps/macos/")
        .join(package_name)
        .join(release_key)
        .join("artifacts");
    base_target_path
}

fn main() {
    wisual_logger::init_from_env();
    log::info!(
        "Starting augmented-dev-cli VERSION={} GIT_REV={} GIT_REV_SHORT={}",
        get_cli_version(),
        env!("GIT_REV"),
        env!("GIT_REV_SHORT")
    );

    let version = get_cli_version();
    let mut app = clap::App::new("augmented-dev-cli")
        .version(&*version)
        .about("Development CLI for augmented projects, helps build and deploy apps")
        .subcommand(
            clap::App::new("build")
                .about("Build a release package for a given app")
                .arg(clap::Arg::from("-c, --crate=<PATH> 'Crate path'")),
        );

    let matches = app.clone().get_matches();
    if matches.is_present("build") {
        let matches = matches.subcommand_matches("build").unwrap();
        run_build_release(matches.value_of("crate").unwrap());
    } else {
        app.print_help().unwrap();
        std::process::exit(1);
    }
}
