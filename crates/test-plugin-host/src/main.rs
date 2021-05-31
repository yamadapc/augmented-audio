use std::process::exit;

mod commands;
mod host;

fn main() {
    wisual_logger::init_from_env();
    let mut app = clap::App::new("test-plugin-host")
        .version("0.0.1")
        .author("Pedro Tacla Yamada <tacla.yamada@gmail.com>")
        .about("Test audio plugins")
        .subcommand(commands::options::build_run_command())
        .subcommand(clap::App::new("list-devices").about("Lists audio devices"));
    let matches = app.clone().get_matches();

    if matches.is_present("list-devices") {
        commands::run_list_devices();
        return;
    }

    if let Some(run_options) = commands::options::parse_run_options(matches) {
        commands::run_test(run_options);
    } else {
        app.print_help().unwrap();
        exit(1);
    }
}
