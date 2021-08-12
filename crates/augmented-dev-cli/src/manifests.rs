use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseNotes {
    pub text: Option<String>,
    pub html: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseJson {
    pub name: String,
    pub key: String,
    pub created_at: String,
    pub release_notes: Option<ReleaseNotes>,
    pub file_download_url: String,
    pub user_download_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type")]
pub enum MacosAppConfig {
    #[serde(rename_all = "kebab-case")]
    AppTemplate { template_path: String },
    #[serde(rename_all = "kebab-case")]
    Vst,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AppConfig {
    pub public_name: String,
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
