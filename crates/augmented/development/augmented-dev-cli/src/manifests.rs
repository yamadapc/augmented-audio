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
use serde::{Deserialize, Serialize};
use toml::value::Table;

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
    pub skip: Option<bool>,
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
    pub dependencies: Option<Table>,
    pub dev_dependencies: Option<Table>,
    pub build_dependencies: Option<Table>,
    pub lib: Option<CargoLib>,
}

impl CargoToml {
    pub fn has_snapshot_tests(&self) -> bool {
        self.package
            .metadata
            .as_ref()
            .and_then(|metadata| {
                metadata.augmented.as_ref().map(|augmented| {
                    augmented.processor_examples.is_some()
                        && !augmented.processor_examples.as_ref().unwrap().is_empty()
                })
            })
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
