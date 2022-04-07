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
