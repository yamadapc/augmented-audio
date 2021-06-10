use std::process::exit;

use plugin_host_lib::commands::options;
use plugin_host_lib::commands::run_list_devices;
use plugin_host_lib::commands::run_test;

fn main() {
    wisual_logger::init_from_env();
    let mut app = clap::App::new("test-plugin-host")
        .version("0.0.1")
        .author("Pedro Tacla Yamada <tacla.yamada@gmail.com>")
        .about("Test audio plugins")
        .subcommand(options::build_run_command())
        .subcommand(clap::App::new("list-devices").about("Lists audio devices"));
    let matches = app.clone().get_matches();

    if matches.is_present("list-devices") {
        run_list_devices();
        return;
    }

    if let Some(run_options) = options::parse_run_options(matches) {
        run_test(run_options);
    } else {
        app.print_help().unwrap();
        exit(1);
    }
}
