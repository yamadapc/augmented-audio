use std::path::Path;

use cmd_lib::run_cmd;
use mockall::automock;

use crate::manifests::{CargoToml, ReleaseJson};
use crate::services::build_command_service::packager_service::LocalPackage;

pub struct ReleaseInput<'a> {
    pub cargo_toml: &'a CargoToml,
    pub local_package: &'a LocalPackage,
    pub release_json: &'a ReleaseJson,
}

#[automock]
pub trait ReleaseService {
    /// Sign compress and upload package
    #[allow(clippy::needless_lifetimes)]
    fn release<'a>(&self, release_input: ReleaseInput<'a>);
}

#[derive(Default)]
pub struct ReleaseServiceImpl {}

impl ReleaseService for ReleaseServiceImpl {
    /// Sign compress and upload package
    fn release<'a>(&self, release_input: ReleaseInput) {
        sign_app(release_input.local_package);
        let artifact_path = compress_release(release_input.cargo_toml, release_input.local_package);
        upload_release(release_input.release_json, artifact_path);
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

fn compress_release(cargo_toml: &CargoToml, local_package: &LocalPackage) -> String {
    let path = &local_package.path;
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
