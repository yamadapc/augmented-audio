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

use crate::services::release_service::prerelease_all_crates;
use crate::services::snapshot_tests_service::run_all_snapshot_tests;

mod logger;
mod manifests;
mod services;

fn get_cli_version() -> String {
    format!(
        "{}-{}-{}",
        env!("PROFILE"),
        env!("CARGO_PKG_VERSION"),
        env!("GIT_REV_SHORT")
    )
}

fn main() {
    logger::try_init_from_env().unwrap();
    log::warn!(
        "Starting augmented-dev-cli VERSION={} GIT_REV={} GIT_REV_SHORT={}",
        get_cli_version(),
        env!("GIT_REV"),
        env!("GIT_REV_SHORT")
    );

    let version = get_cli_version();
    let mut app = clap::Command::new("augmented-dev-cli")
        .subcommand(
            clap::Command::new("list-crates")
                .about("List crates and their published status")
                .arg(
                    clap::Arg::new("simple")
                        .long("simple")
                        .help("Only list names and prevent logs")
                        .short('s')
                        .num_args(0),
                ),
        )
        .subcommand(
            clap::Command::new("prerelease-all")
                .about("Bump all crates into a pre-release state")
                .arg(
                    clap::Arg::new("dry-run")
                        .long("dry-run")
                        .help("Don't run `cargo publish`"),
                ),
        )
        .subcommand(clap::Command::new("outdated").about("List outdated dependencies"))
        .subcommand(
            clap::Command::new("test-snapshots")
                .about("Run processor snapshot tests")
                .arg(
                    clap::Arg::new("update-snapshots")
                        .long("update-snapshots")
                        .short('u'),
                ),
        )
        .subcommand(
            clap::Command::new("build")
                .about("Build a release package for a given app")
                .arg(
                    clap::Arg::new("upload")
                        .long("upload")
                        .short('u')
                        .help("Upload artifacts to S3"),
                )
                .arg(clap::Arg::new("crate").num_args(1)),
        )
        .version(version)
        .about("Development CLI for augmented projects, helps build and deploy apps");

    let matches = app.clone().get_matches();

    if let Some(matches) = matches.subcommand_matches("prerelease-all") {
        let list_crates_service = services::ListCratesService::default();
        let dry_run = matches.get_flag("dry-run");
        prerelease_all_crates(&list_crates_service, dry_run).expect("Failed to prerelease");
    } else if matches.subcommand_matches("outdated").is_some() {
        let outdated_crates_service = services::OutdatedCratesService::default();
        outdated_crates_service.run();
    } else if let Some(matches) = matches.subcommand_matches("list-crates") {
        let list_crates_service = services::ListCratesService::default();
        list_crates_service.run(matches.get_flag("simple"));
    } else if let Some(matches) = matches.subcommand_matches("build") {
        let mut build_service = services::BuildCommandService::default();
        let upload = matches.get_flag("upload");
        let crate_path = matches.get_one::<String>("crate").map(|s| s.as_str());
        build_service.run_build(crate_path, upload);
    } else if let Some(matches) = matches.subcommand_matches("test-snapshots") {
        run_all_snapshot_tests(Default::default(), matches.get_flag("update-snapshots"));
    } else {
        app.print_help().unwrap();
        std::process::exit(1);
    }
}
