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
use std::process::exit;

use plugin_host_lib::commands::options;
use plugin_host_lib::commands::run_list_devices;
use plugin_host_lib::commands::run_test;

fn main() {
    wisual_logger::init_from_env();
    let mut app = clap::App::new("plugin-host")
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

    if let Some(run_options) = matches
        .subcommand_matches("run")
        .map(options::parse_run_options)
        .flatten()
    {
        run_test(run_options);
    } else {
        app.print_help().unwrap();
        exit(1);
    }
}
