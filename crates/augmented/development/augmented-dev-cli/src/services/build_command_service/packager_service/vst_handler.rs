use std::fs::create_dir_all;
use std::path::PathBuf;

use cmd_lib::run_cmd;

use crate::manifests::{CargoToml, VstConfig};
use crate::services::build_command_service::packager_service::{LocalPackage, PackagerInput};

pub struct VstHandler {}

impl VstHandler {
    pub fn handle(
        target_path: PathBuf,
        input: &PackagerInput,
        vst: VstConfig,
    ) -> Option<LocalPackage> {
        let app_config = input
            .cargo_toml
            .package
            .metadata
            .as_ref()
            .unwrap()
            .app
            .as_ref()
            .unwrap();

        let plist_file = build_plist(&app_config.public_name, input.cargo_toml, &vst);
        run_cmd!(mkdir -p ${target_path}).unwrap();

        let output_path = target_path.join(format!("{}.vst", app_config.public_name));
        log::info!("Creating VST at: {}", output_path.to_str().unwrap());
        create_dir_all(output_path.join("Contents/MacOS")).expect("Failed to create directories");

        let output_path = output_path.canonicalize().unwrap();
        let plist_path = output_path.join("Contents/Info.plist");
        log::info!(
            "Writing Info.plist file to: {}",
            plist_path.to_str().unwrap()
        );
        plist_file
            .to_file_xml(plist_path)
            .expect("Failed to write plist file");

        let source_dylib_path = format!(
            "./target/release/lib{}.dylib",
            input
                .cargo_toml
                .lib
                .as_ref()
                .expect("VST requires a lib")
                .name
                .as_ref()
                .expect("VST lib require a name")
        );
        let target_dylib_path =
            output_path.join(format!("Contents/MacOS/{}", input.cargo_toml.package.name));
        log::info!(
            "Copying binary library from: {} to {}",
            &source_dylib_path,
            target_dylib_path.to_str().unwrap(),
        );
        std::fs::copy(source_dylib_path, target_dylib_path).expect("Failed to copy binary lib");

        None
    }
}

fn build_plist(public_name: &str, toml_file: &CargoToml, vst: &VstConfig) -> plist::Value {
    let mut plist_file = plist::Dictionary::new();
    plist_file.insert(
        String::from("CFBundleIdentifier"),
        plist::Value::from(vst.identifier.clone()),
    );
    plist_file.insert(
        String::from("CFBundleName"),
        plist::Value::from(String::from(public_name)),
    );
    plist_file.insert(
        String::from("CFBundleVersion"),
        plist::Value::from(toml_file.package.version.clone()),
    );
    plist_file.insert(
        String::from("CFBundleExecutable"),
        plist::Value::from(toml_file.package.name.clone()),
    );
    plist_file.insert(
        String::from("CFResourcesFileMapped"),
        plist::Value::from(""),
    );
    plist_file.insert(
        String::from("CFBundleGetInfoString"),
        plist::Value::from("vst"),
    );
    plist_file.insert(String::from("CFBundleIconFile"), plist::Value::from(""));
    plist_file.insert(
        String::from("CFBundleInfoDictionaryVersion"),
        plist::Value::from("6.0"),
    );
    plist_file.insert(
        String::from("CFBundlePackageType"),
        plist::Value::from("BNDL"),
    );
    plist_file.insert(
        String::from("CFBundleDevelopmentRegion"),
        plist::Value::from("English"),
    );
    plist::Value::Dictionary(plist_file)
}
