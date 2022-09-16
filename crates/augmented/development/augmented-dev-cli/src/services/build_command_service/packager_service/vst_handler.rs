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
use std::fs::create_dir_all;
use std::path::PathBuf;

use cmd_lib::run_cmd;

use crate::manifests::{CargoToml, VstConfig};
use crate::services::build_command_service::packager_service::{LocalPackage, PackagerInput};

pub struct VstHandler {}

impl VstHandler {
    pub fn handle(
        target_path: PathBuf,
        input: PackagerInput,
        vst: VstConfig,
    ) -> Option<LocalPackage> {
        let public_name = &input.public_name;

        let plist_file = build_plist(public_name, &input.cargo_toml, &vst);
        run_cmd!(mkdir -p ${target_path}).unwrap();

        let output_path = target_path.join(format!("{}.vst", public_name));
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

        let source_dylib_path = input
            .example_name
            .clone()
            .map(|example| format!("./target/release/examples/lib{}.dylib", example))
            .unwrap_or_else(|| {
                format!(
                    "./target/release/lib{}.dylib",
                    input
                        .cargo_toml
                        .lib
                        .as_ref()
                        .expect("VST requires a lib")
                        .name
                        .as_ref()
                        .expect("VST lib require a name")
                )
            });
        let target_dylib_path =
            output_path.join(format!("Contents/MacOS/{}", input.cargo_toml.package.name));
        log::info!(
            "Copying binary library from: {} to {}",
            &source_dylib_path,
            target_dylib_path.to_str().unwrap(),
        );
        std::fs::copy(source_dylib_path, target_dylib_path).expect("Failed to copy binary lib");

        std::fs::write(output_path.join("Contents/PkgInfo"), "BNDL????")
            .expect("Failed to create PkgInfo");

        Some(LocalPackage {
            input,
            path: output_path.to_str().unwrap().to_string(),
            target_app_path: output_path,
        })
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
        String::from("CFBundleLongVersionString"),
        plist::Value::from(toml_file.package.version.clone()),
    );
    plist_file.insert(
        String::from("CFBundleShortVersionString"),
        plist::Value::from(toml_file.package.version.clone()),
    );
    plist_file.insert(
        String::from("CFBundleExecutable"),
        plist::Value::from(toml_file.package.name.clone()),
    );
    plist_file.insert(
        String::from("CFBundleSignature"),
        plist::Value::from(format!("{}", rand::random::<usize>() % 9999)),
    );
    plist_file.insert(
        String::from("CFBundleSupportedPlatforms"),
        plist::Value::from(vec![plist::Value::from("MacOSX")]),
    );
    plist_file.insert(
        String::from("CFBundleGetInfoString"),
        plist::Value::from("vst"),
    );
    plist_file.insert(
        String::from("CSResourcesFileMapped"),
        plist::Value::from(true),
    );

    plist_file.insert(String::from("CFBundleIconFile"), plist::Value::from(""));
    plist_file.insert(
        String::from("CFBundleInfoDictionaryVersion"),
        plist::Value::from("6.0"),
    );
    plist_file.insert(
        String::from("CFBundlePackageType"),
        plist::Value::from("APPL"),
    );
    // plist_file.insert(String::from("DTPlatformName"), plist::Value::from("macosx"));
    // plist_file.insert(
    //     String::from("DTPlatformVersion"),
    //     plist::Value::from("11.1"),
    // );
    // plist_file.insert(
    //     String::from("LSMinimumSystemVersion"),
    //     plist::Value::from("10.10"),
    // );
    plist_file.insert(
        String::from("CFBundleDevelopmentRegion"),
        plist::Value::from("English"),
    );
    plist::Value::Dictionary(plist_file)
}
