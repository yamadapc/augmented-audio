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
pub struct VstConfig {
    pub identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AppTemplateConfig {
    pub template_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type")]
pub enum MacosAppConfig {
    #[serde(rename_all = "kebab-case")]
    AppTemplate(AppTemplateConfig),
    #[serde(rename_all = "kebab-case")]
    Vst(VstConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct AppConfig {
    pub public_name: String,
    pub macos: Option<MacosAppConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AugmentedMetadata {
    pub private: Option<bool>,
    pub processor_examples: Option<Vec<String>>,
    pub vst_examples: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoTomlPackageMetadata {
    pub app: Option<AppConfig>,
    pub augmented: Option<AugmentedMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoTomlPackage {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub metadata: Option<CargoTomlPackageMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CargoLib {
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoToml {
    pub package: CargoTomlPackage,
    pub lib: Option<CargoLib>,
}

impl CargoToml {
    pub fn has_snapshot_tests(&self) -> bool {
        self.package
            .metadata
            .as_ref()
            .map(|metadata| {
                metadata.augmented.as_ref().map(|augmented| {
                    augmented.processor_examples.is_some()
                        && !augmented.processor_examples.as_ref().unwrap().is_empty()
                })
            })
            .flatten()
            .unwrap_or(false)
    }

    pub fn is_augmented_crate(&self) -> bool {
        self.package
            .metadata
            .as_ref()
            .map(|metadata| metadata.augmented.is_some())
            .unwrap_or(false)
    }
}
