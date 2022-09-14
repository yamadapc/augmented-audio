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
use std::fs::read_to_string;
use std::path::Path;

use mockall::automock;

use crate::manifests::CargoToml;

#[automock]
pub trait CargoTomlReader {
    fn read(&self, crate_path: &str) -> CargoToml;
}

#[derive(Default)]
pub struct CargoTomlReaderImpl {}

impl CargoTomlReader for CargoTomlReaderImpl {
    fn read(&self, crate_path: &str) -> CargoToml {
        let config_path = Path::new(crate_path).join("Cargo.toml");
        log::info!("Reading cargo toml at: {:?}", config_path);
        let input_cargo_file = read_to_string(config_path).expect("Failed to read toml file");
        let cargo_toml: CargoToml = toml::from_str(&input_cargo_file).unwrap_or_else(|err| {
            log::error!("Parse error: {}", err);
            panic!("Failed to parse toml file at {}", crate_path)
        });
        cargo_toml
    }
}

#[cfg(test)]
mod test {

    // #[test]
    // fn test_read_cargo() {
    //     let reader = CargoTomlReaderImpl::default();
    //     let toml = reader.read(env!("CARGO_MANIFEST_DIR"));
    //     assert_eq!(toml.package.name, "augmented-dev-cli");
    // }
}
