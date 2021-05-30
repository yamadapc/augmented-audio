mod commands;
mod host;

fn main() {
    wisual_logger::init_from_env();
    let matches = clap::App::new("test-plugin-host")
        .version("0.0.1")
        .author("Pedro Tacla Yamada <tacla.yamada@gmail.com>")
        .about("Test audio plugins")
        .subcommand(commands::options::build_run_command())
        .subcommand(clap::App::new("list-devices").about("Lists audio devices"))
        .get_matches();

    if matches.is_present("list-devices") {
        commands::run_list_devices();
        return;
    }

    let run_options = commands::options::parse_run_options(matches).unwrap();
    commands::run_test(run_options);
}
