use std::fs::{copy, create_dir_all, read_to_string};
use std::path::{Path, PathBuf};

use cmd_lib::run_cmd;

use manifests::CargoToml;

mod manifests;

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

fn build_plist_and_bundle(config_path: &str, output_path: &str) -> PathBuf {
    log::info!(
        "Reading package toml file config_path={} output_path={}",
        config_path,
        output_path
    );
    let input_cargo_file = read_to_string(config_path).expect("Failed to read toml file");
    let toml_file: CargoToml =
        toml::from_str(&input_cargo_file).expect("Failed to parse toml file");
    let name = toml_file
        .package
        .metadata
        .bundle
        .name
        .clone()
        .unwrap_or_else(|| toml_file.package.name.to_string());

    log::info!("Building package plist");
    let plist_file = build_plist(&name, &toml_file);

    let output_path = Path::new(output_path);
    if output_path.exists() {
        log::warn!(
            "Can't create at {}. Path already exists. Will try to continue.",
            output_path.to_str().unwrap()
        );
    }
    create_dir_all(&output_path).expect("Failed to create directory");
    create_dir_all(output_path.join("Contents")).expect("Failed to create directory");
    create_dir_all(output_path.join("Contents/MacOS")).expect("Failed to create directory");
    let output_path = output_path.canonicalize().unwrap();

    let plist_path = output_path.join("Contents/Info.plist");
    log::info!("Writing Info.plist file");
    plist_file
        .to_file_xml(plist_path)
        .expect("Failed to write plist file");

    let package_path = Path::new(config_path).parent().unwrap();
    log::info!("Forcing a build of the package");
    run_cmd!(cd ${package_path}; cargo build --release).unwrap();

    let source_dylib_path =
        find_target(config_path, &toml_file).expect("Couldn't find the target dylib");
    let target_dylib_path = output_path.join(format!("Contents/MacOS/{}", name));
    log::info!(
        "Copying binary library from {} to {}",
        &source_dylib_path,
        target_dylib_path.to_str().unwrap(),
    );
    copy(source_dylib_path, target_dylib_path).expect("Failed to copy binary lib");
    output_path
}

fn build_plist(name: &str, toml_file: &CargoToml) -> plist::Value {
    let mut plist_file = plist::Dictionary::new();
    plist_file.insert(
        String::from("CFBundleIdentifier"),
        plist::Value::from(toml_file.package.metadata.bundle.identifier.clone()),
    );
    plist_file.insert(String::from("CFBundleName"), plist::Value::from(name));
    plist_file.insert(
        String::from("CFBundleVersion"),
        plist::Value::from(toml_file.package.version.clone()),
    );
    plist_file.insert(String::from("CFBundleExecutable"), plist::Value::from(name));
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

fn build_frontend(frontend_path: &str) {
    log::info!("Building front-end");
    let path = Path::new(frontend_path);
    run_cmd!(cd ${path}; yarn run build).unwrap();
    log::info!("Finished building front-end");
}

fn move_frontend_to_output(frontend_path: &str, output_path: &Path) {
    let frontend_build_path = Path::new(frontend_path).join("build");
    log::info!(
        "Copying front-end into bundle path frontend_build_path={} output_path={}",
        frontend_build_path.to_str().unwrap(),
        output_path.to_str().unwrap()
    );
    let resources_directory = output_path.join("Contents/Resources");
    create_dir_all(resources_directory.clone()).expect("Failed to create resources dir");
    run_cmd!(cp -r ${frontend_build_path} ${resources_directory}/frontend)
        .expect("Failed to copy front-end assets");
}

fn main() {
    wisual_logger::init_from_env();

    let app = clap::App::new("ruas-bundler")
        .version("0.1.0")
        .author("Pedro Tacla Yamada <tacla.yamada@gmail.com>")
        .about("Package Rust plug-ins")
        .arg(clap::Arg::from_usage(
            "-c,--config=<PACKAGE_CARGO_TOML> 'Cargo.toml PATH for the package to bundle'",
        ))
        .arg(clap::Arg::from_usage(
            "-o,--output=<OUTPUT_PATH> 'Where to write the plug-in into'",
        ))
        .arg(clap::Arg::from_usage(
            "--frontend-path=[FRONTEND_PATH] 'Front-end path'",
        ))
        .arg(clap::Arg::from_usage(
            "--skip-frontend-build 'Skip the front-end build'",
        ));
    let matches = app.get_matches();
    let config_path = matches
        .value_of("config")
        .expect("Expected Cargo.toml filepath (--config=...)");
    let output_path = matches
        .value_of("output")
        .expect("Expected output path (--output=...)");
    let frontend_path = matches.value_of("frontend-path");

    let output_path = build_plist_and_bundle(config_path, output_path);
    if let Some(frontend_path) = frontend_path {
        if !matches.is_present("skip-frontend-build") {
            build_frontend(frontend_path);
        }
        move_frontend_to_output(frontend_path, &output_path);
    }
}
