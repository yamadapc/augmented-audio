use serde::{Deserialize, Serialize};
use toml::map::Map;
use toml::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseJson {
    pub name: String,
    pub key: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type")]
pub enum MacosAppConfig {
    #[serde(rename_all = "kebab-case")]
    AppTemplate { template_path: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub macos: Option<MacosAppConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoTomlPackageMetadata {
    pub app: Option<AppConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoTomlPackage {
    pub name: String,
    pub description: String,
    pub version: String,
    pub metadata: Option<CargoTomlPackageMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoToml {
    pub package: CargoTomlPackage,
}
