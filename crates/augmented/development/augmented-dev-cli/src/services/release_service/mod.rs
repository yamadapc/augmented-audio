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

use std::path::PathBuf;
use std::time::Duration;

use semver::Version;
use serde::{Deserialize, Serialize};
use toml::Value;
use toml_edit::{Document, Item};

use itertools::Itertools;

use crate::manifests::CargoToml;
use crate::services::cargo_toml_reader::{CargoTomlReader, CargoTomlReaderImpl};
use crate::services::ListCratesService;

pub fn prerelease_all_crates(
    list_crates_service: &ListCratesService,
    dry_run: bool,
) -> anyhow::Result<()> {
    let cargo_toml_reader = CargoTomlReaderImpl::default();
    let all_crates = list_crates_service.find_manifests();
    let crates = list_crates_service.find_augmented_crates();

    for (path, _manifest) in crates {
        let manifest = cargo_toml_reader.read(&path);
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

        let changes = crate_has_changes(&path, &manifest).unwrap_or_else(|_| {
            vec![ChangeRecord {
                commit: "".to_string(),
                summary: "Initial release".to_string(),
                change_level: ChangeLevel::Minor,
            }]
        });

        if !changes.is_empty() {
            prerelease_crate(&path, &manifest, &all_crates, dry_run, &changes)?;

            if !dry_run {
                log::info!(
                    "Waiting for upload to finish. It takes a while before the crate is visible"
                );
                std::thread::sleep(Duration::from_secs(30));
            }
        }
    }

    Ok(())
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
enum ChangeLevel {
    Major = 3,
    Minor = 2,
    Patch = 1,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChangeRecord {
    commit: String,
    summary: String,
    change_level: ChangeLevel,
}

fn crate_has_changes(path: &str, manifest: &CargoToml) -> anyhow::Result<Vec<ChangeRecord>> {
    let previous_version = &manifest.package.version;
    let tag = format!("{}@{}", &*manifest.package.name, previous_version);
    let result = cmd_lib::run_fun!(PAGER= git diff $tag $path)?;
    log::debug!("git diff {} {}\n    ==>\n\n{}\n\n", tag, path, result);
    let commit_list = cmd_lib::run_fun!(PAGER= git log --oneline $tag...HEAD $path)?;
    log::debug!("git log {}...HEAD --oneline {}", tag, path);

    let changes = if result.is_empty() {
        vec![]
    } else {
        let mut changes = vec![];
        for line in commit_list.split('\n') {
            let commit = line.split(' ').take(1).join("");
            let summary = line.split(' ').skip(1).join(" ");
            let change_level = resolve_change_level(&commit, &summary);
            changes.push(ChangeRecord {
                commit,
                summary,
                change_level,
            })
        }
        changes
    };

    if result.is_empty() {
        log::warn!("SKIPPING - NO changes in {}", path);
    } else {
        log::warn!("BUMPING - Found changes in {}", path);
        log::info!("Changes in release:\n{:#?}", changes);
    }

    Ok(changes)
}

fn resolve_change_level(commit: &str, summary: &str) -> ChangeLevel {
    let notes = cmd_lib::run_fun!(git notes show $commit).unwrap_or_else(|_| "".to_string());
    let full_text = format!("{}\n\nNotes:\n{}", summary, notes);

    if full_text.contains(":bug:") || full_text.contains(":patch:") {
        ChangeLevel::Patch
    } else if full_text.contains(":feature:") || full_text.contains(":minor:") {
        ChangeLevel::Minor
    } else if full_text.contains(":breaking:") || full_text.contains(":major:") {
        ChangeLevel::Major
    } else {
        ChangeLevel::Minor
    }
}

fn prerelease_crate(
    path: &str,
    manifest: &CargoToml,
    all_crates: &[(String, CargoToml)],
    dry_run: bool,
    changes: &[ChangeRecord],
) -> anyhow::Result<()> {
    log::info!("Running pre-release proc for {} dry_run={}", path, dry_run);

    let new_version = bump_own_version(&manifest.package.name, path, dry_run, changes)?;

    log::info!(
        "  => New version is {}, will now bump it throughout the repo",
        new_version.to_string()
    );

    for (other_crate_path, _) in all_crates {
        let manifest_path = format!("{}/Cargo.toml", other_crate_path);
        let cargo_manifest_str = std::fs::read_to_string(&manifest_path).unwrap();
        let mut cargo_manifest = cargo_manifest_str.parse::<Document>().unwrap();

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
            dry_run,
        )
    }

    lint_manifest(manifest);
    if !dry_run {
        publish_and_release(path, manifest, new_version);
    }

    Ok(())
}

fn bump_dependency(
    target_package_path: &str,
    target_package_manifest: &mut Document,
    bump_package_name: String,
    bump_package_version: &Version,
    dry_run: bool,
) {
    let other_crate_name = target_package_manifest["package"]["name"]
        .as_str()
        .unwrap()
        .to_string();

    let mut bump_dep = |key| {
        if let Some(deps) = target_package_manifest[key].as_table_mut() {
            if let Some(subdep) = deps.get_mut(&bump_package_name) {
                log::info!(
                    "  => Bumping {}/dependencies/{} to {}",
                    other_crate_name,
                    &bump_package_name,
                    bump_package_version.to_string()
                );
                let current_version = if subdep.is_str() {
                    Version::parse(subdep.as_str().unwrap()).unwrap()
                } else {
                    Version::parse(subdep["version"].as_str().unwrap()).unwrap()
                };
                if current_version.major == bump_package_version.major
                    && current_version.minor == bump_package_version.minor
                {
                    log::warn!("Skipping due to matching major/minor version");
                    return;
                }

                if subdep.is_str() {
                    deps[&*bump_package_name] = value_from_version(bump_package_version);
                } else {
                    subdep["version"] = value_from_version(bump_package_version);
                }

                let cargo_manifest_str = target_package_manifest.to_string();

                if !dry_run {
                    std::fs::write(target_package_path, cargo_manifest_str).unwrap();
                }
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
    cmd_lib::spawn!(cargo clippy --no-deps)
        .unwrap()
        .wait()
        .unwrap();
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
fn bump_own_version(
    name: &str,
    path: &str,
    dry_run: bool,
    changes: &[ChangeRecord],
) -> anyhow::Result<Version> {
    let manifest_path = format!("{}/Cargo.toml", path);
    let cargo_manifest_str = std::fs::read_to_string(&manifest_path).unwrap();
    let mut cargo_manifest = cargo_manifest_str.parse::<Document>().unwrap();
    let version = cargo_manifest["package"]["version"].as_str().unwrap();
    log::info!("  => Found name={} version={}", name, version);

    let sem_version = Version::parse(version).unwrap();
    let next_version = bump_version(sem_version, changes);

    write_changelog(path, name, &next_version, changes)?;

    cargo_manifest["package"]["version"] = value_from_version(&next_version);
    let cargo_manifest_str = cargo_manifest.to_string();

    if !dry_run {
        std::fs::write(&manifest_path, cargo_manifest_str).unwrap();
    }

    Ok(next_version)
}

fn value_from_version(next_version: &Version) -> Item {
    Item::Value(toml_edit::Value::from(next_version.to_string()))
}

/// Bumps the package version based on a set of changes and returns the new version
fn bump_version(sem_version: Version, changes: &[ChangeRecord]) -> Version {
    let change_level = changes
        .iter()
        .map(|c| c.change_level)
        .max()
        .unwrap_or(ChangeLevel::Patch);

    let mut next_version = Version::new(sem_version.major, sem_version.minor, sem_version.patch);
    match change_level {
        ChangeLevel::Minor => {
            next_version.minor += 1;
            next_version.patch = 0;
        }
        ChangeLevel::Major => {
            next_version.major += 1;
            next_version.minor = 0;
            next_version.patch = 0;
        }
        ChangeLevel::Patch => next_version.patch += 1,
    }

    next_version
}

fn write_changelog(
    path: &str,
    _name: &str,
    next_version: &Version,
    changes: &[ChangeRecord],
) -> anyhow::Result<()> {
    use std::fmt::Write;

    // let changelogs_path = PathBuf::from(path).join("./changelogs");
    // std::fs::create_dir_all(&changelogs_path)?;
    // let changelog_file_name = format!("{}@{}.json", name, next_version.to_string());
    // let changelog_path = changelogs_path.join(changelog_file_name);
    // log::info!("Writing {:?}", changelog_path);
    // std::fs::write(changelog_path, serde_json::to_string(changes)?)?;
    // let changelog_file_md = format!("{}@{}.md", name, next_version.to_string());

    let mut changelog_md = format!("## v{}\n\n", next_version);
    for change in changes {
        writeln!(
            changelog_md,
            "* [`{}`](https://github.com/yamadapc/augmented-audio/commits/{}) {} ({:?})",
            change.commit, change.commit, change.summary, change.change_level
        )?;
    }

    // let changelog_path = changelogs_path.join(changelog_file_md);
    // std::fs::write(changelog_path, &changelog_md)?;

    let full_changelog_path = PathBuf::from(path).join("./CHANGELOG.md");
    let changelog_contents =
        std::fs::read_to_string(&full_changelog_path).unwrap_or_else(|_| "".to_string());
    std::fs::write(
        full_changelog_path,
        format!("{}\n{}", changelog_md, changelog_contents),
    )?;

    Ok(())
}

fn lint_manifest(manifest: &CargoToml) {
    // No pre-release dependencies
    if let Some(dependencies) = &manifest.dependencies {
        for (dep, version) in dependencies {
            println!("{} = {}", dep, version);

            assert!(is_valid_version(version), "INVALID DEPENDENCY VERSION")
        }
    }
}

fn is_valid_version(value: &Value) -> bool {
    if let Some(s) = value.as_str() {
        !s.contains("alpha")
    } else if let Some(table_ver) = value.as_table() {
        table_ver
            .get("version")
            .map(is_valid_version)
            .unwrap_or_else(|| table_ver.get("git").is_some())
    } else {
        false
    }
}
