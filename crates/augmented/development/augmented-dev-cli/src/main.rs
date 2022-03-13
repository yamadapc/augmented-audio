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
    log::info!(
        "Starting augmented-dev-cli VERSION={} GIT_REV={} GIT_REV_SHORT={}",
        get_cli_version(),
        env!("GIT_REV"),
        env!("GIT_REV_SHORT")
    );

    let version = get_cli_version();
    let mut app = clap::Command::new("augmented-dev-cli")
        .subcommand(
            clap::Command::new("list-crates").about("List crates and their published status"),
        )
        .subcommand(
            clap::Command::new("prerelease-all").about("Bump all crates into a pre-release state"),
        )
        .subcommand(
            clap::Command::new("test-snapshots")
                .about("Run processor snapshot tests")
                .arg(clap::Arg::new("update-snapshots").short('u')),
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
    } else if matches.subcommand_matches("list-crates").is_some() {
        let list_crates_service = services::ListCratesService::default();
        list_crates_service.run();
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
