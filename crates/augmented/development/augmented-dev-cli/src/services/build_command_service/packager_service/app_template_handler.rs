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
use std::path::{Path, PathBuf};

use cmd_lib::run_cmd;

use crate::manifests::AppTemplateConfig;
use crate::services::build_command_service::packager_service::{LocalPackage, PackagerInput};

pub struct AppTemplateHandler {}

impl AppTemplateHandler {
    pub fn handle(
        base_target_path: PathBuf,
        input: PackagerInput,
        config: AppTemplateConfig,
    ) -> Option<LocalPackage> {
        let template_path = Path::new(&input.crate_path).join(config.template_path);

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
            input,
            target_app_path,
        })
    }
}
