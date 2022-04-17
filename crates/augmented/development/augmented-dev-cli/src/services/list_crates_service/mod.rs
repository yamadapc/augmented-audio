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
mod dependency_graph;

use std::collections::{HashMap, HashSet};
use std::fs::read_dir;

use crate::manifests::CargoToml;
use crate::services::cargo_toml_reader::{CargoTomlReader, CargoTomlReaderImpl};
use crate::services::list_crates_service::dependency_graph::DependencyGraph;
use crates_io_api::SyncClient;

/// Can list crates in the mono-repo and also list only the augmented audio crates.
///
/// Crates are considered part of augmented if they declare `[package.metadata.augmented]`.
pub struct ListCratesService {
    cargo_toml_reader: Box<dyn CargoTomlReader>,
    client: SyncClient,
}

impl Default for ListCratesService {
    fn default() -> Self {
        let client = SyncClient::new("augmented-dev-cli", std::time::Duration::from_millis(1000))
            .expect("Failed to create crates.io API client");

        ListCratesService {
            cargo_toml_reader: Box::new(CargoTomlReaderImpl::default()),
            client,
        }
    }
}

impl ListCratesService {
    pub fn run(&self) {
        log::info!("Finding crates...");

        let manifests = self.find_manifests();

        for (_, manifest) in manifests {
            self.run_get_info(manifest);
        }
    }

    pub fn find_augmented_crates(&self) -> Vec<(String, CargoToml)> {
        self.find_manifests()
            .into_iter()
            .filter(|(_, manifest)| manifest.is_augmented_crate())
            .collect()
    }

    pub fn find_manifests(&self) -> Vec<(String, CargoToml)> {
        let crates = self.find_entries();
        let result = self.parse_manifests(crates);
        Self::order_crates(&result)
    }

    fn find_entries(&self) -> Vec<String> {
        let mut crates = Vec::new();
        self.find_entries_inner("./crates", &mut crates);
        crates
    }

    fn find_entries_inner(&self, root: &str, crates: &mut Vec<String>) {
        log::debug!("Scanning {}", root);
        let ignore_dirs: HashSet<&str> = ["vendor", "target"].iter().copied().collect();

        let entries =
            read_dir(root).unwrap_or_else(|_| panic!("Failed to list {} directory", root));
        let entries: Vec<_> = entries.into_iter().collect();

        let cargo_manifest = entries.iter().find(|entry| {
            let entry = entry.as_ref().expect("Failed to get DirEntry");
            let file_name = entry.file_name();
            let file_name = file_name.to_str().unwrap();
            file_name == "Cargo.toml"
        });
        if cargo_manifest.is_some() {
            log::debug!("Manifest found at {}", root);
            crates.push(root.into());
            return;
        }

        // Recursive search
        for entry in entries {
            let entry = entry.expect("Failed to get DirEntry");
            let file_name = entry.file_name();
            let file_name = file_name.to_str().unwrap();
            let is_dir = entry.file_type().unwrap().is_dir();

            if is_dir && !ignore_dirs.contains(file_name) {
                self.find_entries_inner(&format!("{}/{}", root, file_name), crates);
            }
        }
    }

    fn parse_manifests(&self, crates: Vec<String>) -> Vec<(String, CargoToml)> {
        crates
            .into_iter()
            .map(|c| {
                let cargo_toml = self.cargo_toml_reader.read(&c);
                (c, cargo_toml)
            })
            .collect()
    }

    fn run_get_info(&self, manifest: CargoToml) {
        let package = manifest.package;
        log::debug!(
            "CRATE - {}@{} - {}",
            package.name,
            package.version,
            package
                .description
                .unwrap_or_else(|| "No description".into())
        );

        let is_private_package = package
            .metadata
            .and_then(|m| m.augmented.map(|a| a.private))
            .flatten()
            .unwrap_or(false);
        if is_private_package {
            return;
        }

        let published_crate = self.client.get_crate(&package.name);
        match published_crate {
            Ok(published_crate) => {
                if published_crate.crate_data.max_version != package.version {
                    log::warn!(
                        "Published version mismatch for {}: local {} <-> crates {}",
                        package.name,
                        package.version,
                        published_crate.crate_data.max_version,
                    );
                } else {
                    log::warn!(
                        "{} crates.io version {} <-> {}",
                        package.name,
                        published_crate.crate_data.max_version,
                        package.version
                    );
                }
            }
            Err(crates_io_api::Error::NotFound(_)) => {
                log::warn!("Crate is not published {}", package.name);
            }
            Err(err) => {
                log::error!("Failed to fetch crate {}: {}", package.name, err);
                panic!("Failed to list crates");
            }
        }
    }

    fn order_crates(result: &[(String, CargoToml)]) -> Vec<(String, CargoToml)> {
        let mut graph = DependencyGraph::default();
        for (_path, manifest) in result {
            graph.add_crate(&manifest.package.name);
        }

        let mut dependency_map: HashMap<String, Vec<String>> = HashMap::new();
        for (path, _manifest) in result {
            Self::add_crate_to_graph(path, &mut dependency_map, &mut graph);
        }
        let ordered_crates = graph.order_crates();
        let result_map: HashMap<String, (String, CargoToml)> = result
            .iter()
            .map(|(path, manifest)| {
                (
                    manifest.package.name.clone(),
                    (path.clone(), manifest.clone()),
                )
            })
            .collect();

        for target in &ordered_crates {
            let deps_list = &dependency_map[target];
            log::info!("{} - Dependencies: {:?}", target, deps_list)
        }

        ordered_crates
            .into_iter()
            .map(|name| result_map[&name].clone())
            .collect()
    }

    fn add_crate_to_graph(
        path: &str,
        dependency_map: &mut HashMap<String, Vec<String>>,
        graph: &mut DependencyGraph,
    ) {
        let manifest_path = format!("{}/Cargo.toml", path);
        let manifest = std::fs::read_to_string(&manifest_path).unwrap();
        let manifest = manifest.parse::<toml_edit::Document>().unwrap();
        let target = manifest["package"]["name"].as_str().unwrap();
        let mut deps_list = vec![];

        let mut add_dep = |dep| {
            if graph.has_crate(dep) {
                graph.add_dependency(target, dep);
                deps_list.push(dep.into());
            }
        };

        if manifest.contains_key("dependencies") {
            for (dep, _spec) in manifest["dependencies"].as_table().unwrap().iter() {
                add_dep(dep);
            }
        }
        if manifest.contains_key("dev-dependencies") {
            for (dep, _spec) in manifest["dev-dependencies"].as_table().unwrap().iter() {
                add_dep(dep);
            }
        }

        dependency_map.insert(target.into(), deps_list);
    }
}
