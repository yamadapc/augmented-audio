use crate::manifests::CargoToml;
use crate::services::ListCratesService;
use semver::{Prerelease, Version};
use toml_edit::Item;

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

        prerelease_crate(&path, &manifest, &all_crates);
    }
}

fn prerelease_crate(path: &str, manifest: &CargoToml, all_crates: &Vec<(String, CargoToml)>) {
    log::info!("Running pre-release proc for {}", path);

    let new_version = bump_prerelease(&manifest.package.name, path);

    log::info!(
        "  => New version is {}, will now bump it throughout the repo",
        new_version.to_string()
    );

    for (other_crate_path, _) in all_crates {
        let manifest_path = format!("{}/Cargo.toml", other_crate_path);
        let cargo_manifest_str = std::fs::read_to_string(&manifest_path).unwrap();
        let mut cargo_manifest = cargo_manifest_str.parse::<toml_edit::Document>().unwrap();

        if cargo_manifest.get("dependencies").is_none() {
            continue;
        }

        let other_crate_name = cargo_manifest["package"]["name"]
            .as_str()
            .unwrap()
            .to_string();
        if let Some(deps) = cargo_manifest["dependencies"].as_table_mut() {
            if let Some(subdep) = deps.get_mut(&*manifest.package.name) {
                log::info!(
                    "  => Bumping {}/dependencies/{} to {}",
                    other_crate_name,
                    &manifest.package.name,
                    new_version.to_string()
                );

                if subdep.is_str() {
                    deps[&*manifest.package.name] = value_from_version(&new_version);
                } else {
                    subdep["version"] = value_from_version(&new_version);
                }

                let cargo_manifest_str = cargo_manifest.to_string();
                std::fs::write(&manifest_path, cargo_manifest_str).unwrap();
            }
        }
    }
}

fn bump_prerelease(name: &str, path: &str) -> Version {
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
        let old_bump: i32 = sem_version.pre.split(".").collect::<Vec<&str>>()[1]
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
