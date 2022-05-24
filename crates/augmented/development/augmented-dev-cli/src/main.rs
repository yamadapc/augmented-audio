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
    wisual_logger::init_from_env();
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
                        .short('s'),
                ),
        )
        .subcommand(
            clap::Command::new("prerelease-all").about("Bump all crates into a pre-release state"),
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
                .arg(clap::Arg::new("crate").takes_value(true)),
        )
        .version(&*version)
        .about("Development CLI for augmented projects, helps build and deploy apps");

    let matches = app.clone().get_matches();

    if matches.subcommand_matches("prerelease-all").is_some() {
        let list_crates_service = services::ListCratesService::default();
        prerelease_all_crates(&list_crates_service);
    } else if matches.subcommand_matches("outdated").is_some() {
        let outdated_crates_service = services::OutdatedCratesService::default();
        outdated_crates_service.run();
    } else if let Some(matches) = matches.subcommand_matches("list-crates") {
        let list_crates_service = services::ListCratesService::default();
        list_crates_service.run(matches.is_present("simple"));
    } else if let Some(matches) = matches.subcommand_matches("build") {
        let mut build_service = services::BuildCommandService::default();
        let crate_path = matches.value_of("crate").unwrap();

        build_service.run_build(crate_path);
    } else if let Some(matches) = matches.subcommand_matches("test-snapshots") {
        run_all_snapshot_tests(Default::default(), matches.is_present("update-snapshots"));
    } else {
        app.print_help().unwrap();
        std::process::exit(1);
    }
}
