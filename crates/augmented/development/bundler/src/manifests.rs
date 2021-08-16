use serde::{Deserialize, Serialize};
use toml::map::Map;
use toml::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MacOsBundleMetadata {
    pub properties: Map<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BundleMetadata {
    pub name: Option<String>,
    pub identifier: String,
    pub macos: Option<MacOsBundleMetadata>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CargoTomlPackageMetadata {
    pub bundle: BundleMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CargoTomlPackage {
    pub name: String,
    pub description: String,
    pub version: String,
    pub metadata: CargoTomlPackageMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CargoLib {
    pub name: String,
    pub crate_type: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CargoToml {
    pub package: CargoTomlPackage,
    pub lib: CargoLib,
}
