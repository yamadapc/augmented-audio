use crate::services::ListCratesService;
use std::collections::{HashMap, HashSet};
use toml::value::Table;
use toml::Value;

#[derive(Debug)]
struct ManifestDependency {
    source_package: String,
    name: String,
    version: String,
}

impl ManifestDependency {
    pub fn from_dependencies_table(source_package: &str, table: &Table) -> Vec<ManifestDependency> {
        let mut result = vec![];
        for dependency in table.keys() {
            if let Some(version) = Self::find_version(table.get(dependency).unwrap()) {
                result.push(ManifestDependency {
                    source_package: source_package.to_string(),
                    name: dependency.to_string(),
                    version,
                });
            }
        }
        result
    }

    fn find_version(value: &Value) -> Option<String> {
        if let Some(s) = value.as_str() {
            Some(s.to_string())
        } else if let Some(t) = value.as_table() {
            t.get("version")
                .map(|s| s.as_str())
                .flatten()
                .map(|s| s.to_string())
        } else {
            None
        }
    }
}

pub struct OutdatedCratesService {
    client: crates_io_api::SyncClient,
}

impl Default for OutdatedCratesService {
    fn default() -> Self {
        let client = crates_io_api::SyncClient::new(
            "augmented-dev-cli",
            std::time::Duration::from_millis(1000),
        )
        .expect("Failed to create crates.io API client");
        Self { client }
    }
}

impl OutdatedCratesService {
    pub fn run(&self) {
        let list_crates_service = ListCratesService::default();
        let augmented_crates = list_crates_service.find_augmented_crates();
        let internal_crates: HashSet<String> = augmented_crates
            .iter()
            .map(|(_, manifest)| manifest.package.name.clone())
            .collect();

        let dependencies = augmented_crates
            .iter()
            .map(|(_pth, manifest)| {
                manifest
                    .dependencies
                    .as_ref()
                    .map(|deps| {
                        ManifestDependency::from_dependencies_table(&manifest.package.name, deps)
                    })
                    .unwrap_or(vec![])
            })
            .flatten()
            .filter(|dependency| !internal_crates.contains(&dependency.name))
            .collect::<Vec<ManifestDependency>>();

        let mut info_cache = HashMap::new();
        for dependency in dependencies {
            let published_dependency = {
                if let Some(dep) = info_cache.get(&dependency.name) {
                    dep
                } else {
                    let dep = self.client.get_crate(&dependency.name).unwrap();
                    info_cache.insert(dependency.name.clone(), dep);
                    info_cache.get(&dependency.name).unwrap()
                }
            };
            let latest_version = {
                let mut vs = published_dependency
                    .versions
                    .iter()
                    .map(|v| semver::Version::parse(&v.num).unwrap())
                    .filter(|v| v.pre.is_empty())
                    .collect::<Vec<semver::Version>>();
                vs.sort();
                vs.get(vs.len() - 1).unwrap().clone()
            };
            let version_req = semver::VersionReq::parse(&dependency.version).unwrap();
            if !version_req.matches(&latest_version) {
                log::warn!(
                    "OUTDATED Source: {} Dependency: {} Version: {} Latest version: {}",
                    &dependency.source_package,
                    &dependency.name,
                    &dependency.version,
                    latest_version,
                );
            } else {
                log::debug!(
                    "OK Source: {} Dependency: {} Version: {} Latest version: {}",
                    &dependency.source_package,
                    &dependency.name,
                    &dependency.version,
                    latest_version,
                );
            }
        }
    }
}