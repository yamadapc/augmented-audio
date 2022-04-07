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
use thiserror::Error;

#[derive(Serialize, Clone, Default, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HostState {
    pub plugin_path: Option<String>,
    pub audio_input_file_path: Option<String>,
}

#[derive(Error, Debug)]
pub enum HostOptionsServiceError {
    #[error("Failed to serialize value")]
    SerdeError(#[from] serde_json::Error),
    #[error("IO error")]
    IOError(#[from] std::io::Error),
}

pub struct HostOptionsService {
    storage_path: String,
}

impl HostOptionsService {
    pub fn new(storage_path: String) -> Self {
        HostOptionsService { storage_path }
    }
}

impl HostOptionsService {
    pub fn fetch(&self) -> Result<HostState, HostOptionsServiceError> {
        let file = std::fs::File::open(&self.storage_path)?;
        Ok(serde_json::from_reader(file)?)
    }

    pub fn store(&self, state: &HostState) -> Result<(), HostOptionsServiceError> {
        let file = std::fs::File::create(&self.storage_path)?;
        Ok(serde_json::to_writer_pretty(file, state)?)
    }
}
