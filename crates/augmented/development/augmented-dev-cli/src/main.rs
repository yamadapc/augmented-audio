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
            clap::App::new("test-snapshots")
                .about("Run processor snapshot tests")
                .arg(clap::Arg::from("-u, --update-snapshots")),
        )
        .subcommand(
            clap::App::new("build")
                .about("Build a release package for a given app")
                .arg(clap::Arg::from("--example=[EXAMPLE] 'Example name'"))
                .arg(clap::Arg::from("-c, --crate=<PATH> 'Crate path'")),
        );

    let matches = app.clone().get_matches();

    if matches.is_present("list-crates") {
        let list_crates_service = services::ListCratesService::default();
        list_crates_service.run();
    } else if matches.is_present("build") {
        let matches = matches.subcommand_matches("build").unwrap();
        let mut build_service = services::BuildCommandService::default();
        let crate_path = matches.value_of("crate").unwrap();
        let example_name = matches.value_of("example");

        build_service.run_build(crate_path, example_name);
    } else if matches.is_present("test-snapshots") {
        let matches = matches.subcommand_matches("test-snapshots").unwrap();
        run_all_snapshot_tests(Default::default(), matches.is_present("update-snapshots"));
    } else {
        app.print_help().unwrap();
        std::process::exit(1);
    }
}
