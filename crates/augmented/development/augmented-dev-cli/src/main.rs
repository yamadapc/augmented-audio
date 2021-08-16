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
        .subcommand(
            clap::App::new("build")
                .about("Build a release package for a given app")
                .arg(clap::Arg::from("-c, --crate=<PATH> 'Crate path'")),
        );

    let matches = app.clone().get_matches();

    if matches.is_present("build") {
        let matches = matches.subcommand_matches("build").unwrap();
        let mut build_service = services::build_command_service::BuildCommandService::default();
        build_service.run_build(matches.value_of("crate").unwrap());
    } else {
        app.print_help().unwrap();
        std::process::exit(1);
    }
}
