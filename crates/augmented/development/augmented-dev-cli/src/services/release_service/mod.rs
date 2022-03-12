use crate::manifests::CargoToml;
use crate::services::ListCratesService;
use semver::{Prerelease, Version};
use toml_edit::{Document, Item};

pub fn prerelease_all_crates(list_crates_service: &ListCratesService) {
    let all_crates = list_crates_service.find_manifests();
    let crates = list_crates_service.find_augmented_crates();

    for (path, manifest) in crates {
        let augmented_metadata = manifest
            .package
            .metadata
            .as_ref()
            .unwrap()
            .augmented
            .as_ref()
            .unwrap();
        if augmented_metadata.private.unwrap_or(true) {
            continue;
        }

        if crate_has_changes(&path, &manifest) {
            prerelease_crate(&path, &manifest, &all_crates);
        }
    }
}

fn crate_has_changes(path: &str, manifest: &CargoToml) -> bool {
    let tag = format!("{}@{}", &*manifest.package.name, &*manifest.package.version);
    let result = cmd_lib::run_fun!(PAGER= git diff $tag $path);
    log::info!("git diff {} {}\n    ==> {:?}", tag, path, result);

    match result {
        Ok(s) if s.is_empty() => {
            log::warn!("SKIPPING - NO changes in {}", path);
            false
        }
        _ => {
            log::warn!("BUMPING - Found changes in {}", path);
            true
        }
    }
}

fn prerelease_crate(path: &str, manifest: &CargoToml, all_crates: &Vec<(String, CargoToml)>) {
    log::info!("Running pre-release proc for {}", path);

    let new_version = bump_own_version_prerelease(&manifest.package.name, path);

    log::info!(
        "  => New version is {}, will now bump it throughout the repo",
        new_version.to_string()
    );

    for (other_crate_path, _) in all_crates {
        let manifest_path = format!("{}/Cargo.toml", other_crate_path);
        let cargo_manifest_str = std::fs::read_to_string(&manifest_path).unwrap();
        let mut cargo_manifest = cargo_manifest_str.parse::<toml_edit::Document>().unwrap();

        if cargo_manifest.get("dependencies").is_none()
            && cargo_manifest.get("dev-dependencies").is_none()
        {
            continue;
        }

        let package_name = manifest.package.name.clone();

        bump_dependency(
            &manifest_path,
            &mut cargo_manifest,
            package_name,
            &new_version,
        )
    }

    publish_and_release(path, manifest, new_version);
}

fn bump_dependency(
    target_package_path: &String,
    target_package_manifest: &mut Document,
    bump_package_name: String,
    bump_package_version: &Version,
) {
    let other_crate_name = target_package_manifest["package"]["name"]
        .as_str()
        .unwrap()
        .to_string();

    let mut bump_dep = |key| {
        if let Some(deps) = target_package_manifest[key].as_table_mut() {
            if let Some(subdep) = deps.get_mut(&*bump_package_name) {
                log::info!(
                    "  => Bumping {}/dependencies/{} to {}",
                    other_crate_name,
                    &bump_package_name,
                    bump_package_version.to_string()
                );

                if subdep.is_str() {
                    deps[&*bump_package_name] = value_from_version(bump_package_version);
                } else {
                    subdep["version"] = value_from_version(bump_package_version);
                }

                let cargo_manifest_str = target_package_manifest.to_string();
                std::fs::write(&target_package_path, cargo_manifest_str).unwrap();
            }
        }
    };

    bump_dep("dependencies");
    bump_dep("dev-dependencies");
}

/// Run unit-tests and publish crate
fn publish_and_release(path: &str, manifest: &CargoToml, new_version: Version) {
    let current = std::env::current_dir().unwrap();
    log::info!("cd {}", path);
    std::env::set_current_dir(path).unwrap();
    log::info!("cargo test");
    cmd_lib::spawn!(cargo test).unwrap().wait().unwrap();
    log::info!("cargo check");
    cmd_lib::spawn!(cargo check).unwrap().wait().unwrap();
    log::info!("cargo clippy");
    cmd_lib::spawn!(cargo clippy).unwrap().wait().unwrap();
    log::info!("cargo build");
    cmd_lib::spawn!(cargo build).unwrap().wait().unwrap();
    log::info!("cargo publish --dry-run --allow-dirty");
    cmd_lib::spawn!(cargo publish --dry-run --allow-dirty)
        .unwrap()
        .wait()
        .unwrap();
    std::env::set_current_dir(&current).unwrap();

    let commit_message = format!("{}@{}", manifest.package.name, new_version);
    cmd_lib::spawn!(git add .).unwrap().wait().unwrap();
    cmd_lib::spawn!(git commit -m "$commit_message")
        .unwrap()
        .wait()
        .unwrap();
    cmd_lib::spawn!(git tag $commit_message)
        .unwrap()
        .wait()
        .unwrap();

    std::env::set_current_dir(path).unwrap();
    log::info!("cargo publish");
    cmd_lib::spawn!(cargo publish).unwrap().wait().unwrap();
    std::env::set_current_dir(&current).unwrap();
}

/// Modify version field in a certain manifest to be bumped to the next pre-release major version
fn bump_own_version_prerelease(name: &str, path: &str) -> Version {
    let manifest_path = format!("{}/Cargo.toml", path);
    let cargo_manifest_str = std::fs::read_to_string(&manifest_path).unwrap();
    let mut cargo_manifest = cargo_manifest_str.parse::<toml_edit::Document>().unwrap();
    let version = cargo_manifest["package"]["version"].as_str().unwrap();
    log::info!("  => Found name={} version={}", name, version);

    let sem_version = Version::parse(version).unwrap();
    let next_version = prerelease_bump(sem_version);

    cargo_manifest["package"]["version"] = value_from_version(&next_version);
    let cargo_manifest_str = cargo_manifest.to_string();
    std::fs::write(&manifest_path, cargo_manifest_str).unwrap();

    next_version
}

fn value_from_version(next_version: &Version) -> Item {
    toml_edit::Item::Value(toml_edit::Value::from(next_version.to_string()))
}

/// Bumps the major version and sets a pre-release alpha counter or bumps the pre-release counter
fn prerelease_bump(sem_version: Version) -> Version {
    let next_version = if !sem_version.pre.is_empty() {
        let mut next_version =
            Version::new(sem_version.major, sem_version.minor, sem_version.patch);
        let old_bump: i32 = sem_version.pre.split('.').collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();
        next_version.pre = Prerelease::new(&*format!("alpha.{}", old_bump + 1)).unwrap();
        next_version
    } else {
        let mut next_version = Version::new(sem_version.major + 1, 0, 0);
        next_version.pre = Prerelease::new("alpha.1").unwrap();
        next_version
    };
    next_version
}
