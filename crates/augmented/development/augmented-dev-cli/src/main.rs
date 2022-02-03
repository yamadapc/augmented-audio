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
    let mut app = clap::App::new("augmented-dev-cli")
        .version(&*version)
        .about("Development CLI for augmented projects, helps build and deploy apps")
        .subcommand(clap::App::new("list-crates").about("List crates and their published status"))
        .subcommand(
            clap::App::new("prerelease-all").about("Bump all crates into a pre-release state"),
        )
        .subcommand(
            clap::App::new("test-snapshots")
                .about("Run processor snapshot tests")
                .arg(clap::Arg::from_usage("-u, --update-snapshots")),
        )
        .subcommand(
            clap::App::new("build")
                .about("Build a release package for a given app")
                .arg(clap::Arg::from_usage("-c, --crate=<PATH> 'Crate path'")),
        );

    let matches = app.clone().get_matches();

    if matches.is_present("prerelease-all") {
        let list_crates_service = services::ListCratesService::default();
        prerelease_all_crates(&list_crates_service);
    } else if matches.is_present("list-crates") {
        let list_crates_service = services::ListCratesService::default();
        list_crates_service.run();
    } else if matches.is_present("build") {
        let matches = matches.subcommand_matches("build").unwrap();
        let mut build_service = services::BuildCommandService::default();
        let crate_path = matches.value_of("crate").unwrap();

        build_service.run_build(crate_path);
    } else if matches.is_present("test-snapshots") {
        let matches = matches.subcommand_matches("test-snapshots").unwrap();
        run_all_snapshot_tests(Default::default(), matches.is_present("update-snapshots"));
    } else {
        app.print_help().unwrap();
        std::process::exit(1);
    }
}
