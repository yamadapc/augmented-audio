use std::path::{Path, PathBuf};

use cmd_lib::run_cmd;

use crate::manifests::AppTemplateConfig;
use crate::services::build_command_service::packager_service::{LocalPackage, PackagerInput};

pub struct AppTemplateHandler {}

impl AppTemplateHandler {
    pub fn handle(
        base_target_path: PathBuf,
        input: &PackagerInput,
        config: AppTemplateConfig,
    ) -> Option<LocalPackage> {
        let template_path = Path::new(input.crate_path).join(config.template_path);

        log::info!(
            "Copying template into `{}` directory",
            base_target_path.to_str().unwrap()
        );
        run_cmd!(mkdir -p ${base_target_path}).unwrap();
        run_cmd!(cp -r ${template_path} ${base_target_path}).unwrap();

        let release_path =
            Path::new("./target/release/").join(input.cargo_toml.package.name.clone());
        let target_app_path = base_target_path.join(template_path.file_name().unwrap());
        run_cmd!(cp ${release_path} ${target_app_path}/Contents/MacOS/).unwrap();

        let release_json_path = base_target_path.join("release.json");
        log::info!("Outputting {}", release_json_path.to_str().unwrap());
        let release_json_str = serde_json::to_string_pretty(&input.release_json).unwrap();
        std::fs::write(release_json_path, release_json_str).unwrap();

        Some(LocalPackage {
            path: base_target_path.to_str().unwrap().to_string(),
            target_app_path,
        })
    }
}
