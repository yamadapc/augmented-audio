use std::fs::read_to_string;
use std::path::Path;

use mockall::automock;

use crate::manifests::CargoToml;

#[automock]
pub trait CargoTomlReader {
    fn read(&self, crate_path: &str) -> CargoToml;
}

pub struct CargoTomlReaderImpl {}

impl Default for CargoTomlReaderImpl {
    fn default() -> Self {
        Self {}
    }
}

impl CargoTomlReader for CargoTomlReaderImpl {
    fn read(&self, crate_path: &str) -> CargoToml {
        let config_path = Path::new(crate_path).join("../../../../../../../Cargo.toml");
        let input_cargo_file = read_to_string(config_path).expect("Failed to read toml file");
        let cargo_toml: CargoToml =
            toml::from_str(&input_cargo_file).expect("Failed to parse toml file");
        cargo_toml
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_cargo() {
        let reader = CargoTomlReaderImpl::default();
        let toml = reader.read(env!("CARGO_MANIFEST_DIR"));
        assert_eq!(toml.package.name, "augmented-dev-cli");
    }
}
