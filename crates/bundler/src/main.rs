use serde::{Deserialize, Serialize};
use std::path::Path;
use toml::map::Map;
use toml::Value;

#[derive(Debug, Serialize, Deserialize)]
struct MacOsBundleMetadata {
    properties: Map<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct BundleMetadata {
    name: Option<String>,
    identifier: String,
    macos: Option<MacOsBundleMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CargoTomlPackageMetadata {
    bundle: BundleMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
struct CargoTomlPackage {
    name: String,
    description: String,
    version: String,
    metadata: CargoTomlPackageMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct CargoLib {
    name: String,
    crate_type: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CargoToml {
    package: CargoTomlPackage,
    lib: CargoLib,
}

fn find_target(config_path: &str, cargo_package: &CargoToml) -> Option<String> {
    let config_path = std::fs::canonicalize(Path::new(config_path)).ok()?;
    let mut config_dir = config_path.parent()?;
    loop {
        log::info!("Searching for target in {:?}", config_dir);
        let mut read_dir = config_dir.read_dir().ok()?;
        let target_dir =
            read_dir.find(|item| item.is_ok() && item.as_ref().unwrap().file_name() == "target");
        if let Some(Ok(target_dir)) = target_dir {
            return Some(String::from(
                target_dir
                    .path()
                    .join(format!("release/lib{}.dylib", cargo_package.lib.name))
                    .to_str()?,
            ));
        } else {
            config_dir = config_dir.parent()?;
        }
    }
}

fn generate(config_path: &str, output_path: &str) {
    log::info!("Reading package toml file");
    let input_cargo_file = std::fs::read_to_string(config_path).expect("Failed to read toml file");
    let toml_file: CargoToml =
        toml::from_str(&input_cargo_file).expect("Failed to parse toml file");

    log::info!("Building package plist");
    let mut plist_file = plist::Dictionary::new();
    let name = toml_file
        .package
        .metadata
        .bundle
        .name
        .clone()
        .unwrap_or(toml_file.package.name.to_string());
    plist_file.insert(
        String::from("CFBundleName"),
        plist::Value::from(name.clone()),
    );
    plist_file.insert(
        String::from("CFBundleVersion"),
        plist::Value::from(toml_file.package.version.clone()),
    );
    plist_file.insert(
        String::from("CFBundleExecutable"),
        plist::Value::from(name.clone()),
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

    let output_path = Path::new(output_path);
    if output_path.exists() {
        log::error!(
            "Can't create at {}. Path already exists.",
            output_path.to_str().unwrap()
        );
    }
    std::fs::create_dir_all(output_path).expect("Failed to create directory");
    std::fs::create_dir_all(output_path.join("Contents")).expect("Failed to create directory");
    std::fs::create_dir_all(output_path.join("Contents/MacOS"))
        .expect("Failed to create directory");
    let plist_path = output_path.join("Contents/Info.plist");
    let plist_file = plist::Value::Dictionary(plist_file);
    log::info!("Writing Info.plist file");
    plist_file
        .to_file_xml(plist_path)
        .expect("Failed to write plist file");

    log::info!("Copying binary library");
    let binary_path = output_path.join(format!("Contents/MacOS/{}", name));
    let target_path = find_target(config_path, &toml_file).expect("Couldn't find the target dylib");
    std::fs::copy(target_path, binary_path).expect("Failed to copy binary lib");
}

fn main() {
    wisual_logger::init_from_env();

    let app = clap::App::new("ruas-bundler")
        .version("0.1.0")
        .author("Pedro Tacla Yamada <tacla.yamada@gmail.com>")
        .about("Package Rust plug-ins")
        .arg(clap::Arg::from_usage(
            "-p,--package=<PACKAGE> 'Package to bundle'",
        ))
        .arg(clap::Arg::from_usage(
            "-o,--output=<OUTPUT_PATH> 'Where to write the plug-in into'",
        ));
    let matches = app.get_matches();
    let config_path = matches
        .value_of("package")
        .expect("Expected package (--package=...)");
    let output_path = matches
        .value_of("output")
        .expect("Expected output path (--output=...)");

    generate(config_path, output_path);
}
