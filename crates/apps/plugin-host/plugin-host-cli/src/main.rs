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

use std::ffi::OsString;
use std::process::exit;

use clap::App;
use mockall::automock;

use plugin_host_lib::commands::options;
use plugin_host_lib::commands::options::RunOptions;
use plugin_host_lib::commands::run_list_devices;
use plugin_host_lib::commands::run_test;

fn main() {
    let args = &mut std::env::args_os();
    run(RunnerImpl {}, args);
}

#[automock]
trait Runner {
    fn run_list_devices(&self);
    fn run_test(&self, run_options: RunOptions);
    fn print_help<'a, 'b>(&self, app: App<'a, 'b>);
}

struct RunnerImpl {}

impl Runner for RunnerImpl {
    fn run_list_devices(&self) {
        run_list_devices();
    }

    fn run_test(&self, run_options: RunOptions) {
        run_test(run_options);
    }

    fn print_help(&self, mut app: App) {
        app.print_help().unwrap();
        exit(1);
    }
}

fn run<T, I>(runner: impl Runner, args: I)
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    wisual_logger::init_from_env();
    let app = clap::App::new("plugin-host")
        .version("0.0.1")
        .author("Pedro Tacla Yamada <tacla.yamada@gmail.com>")
        .about("Test audio plugins")
        .subcommand(options::build_run_command())
        .subcommand(clap::App::new("list-devices").about("Lists audio devices"));
    let matches = app.clone().get_matches_from(args);

    if matches.is_present("list-devices") {
        runner.run_list_devices();
        return;
    }

    if let Some(run_options) = matches
        .subcommand_matches("run")
        .and_then(options::parse_run_options)
    {
        runner.run_test(run_options);
    } else {
        runner.print_help(app);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_list_devices() {
        let mut runner = MockRunner::new();
        runner.expect_run_list_devices().times(1).returning(|| ());
        let args = vec!["cli", "list-devices"];
        run(runner, args);
    }

    #[test]
    fn test_run_test() {
        let mut runner = MockRunner::new();
        runner.expect_run_test().times(1).returning(|_| ());
        let args = vec!["cli", "run", "--plugin", "test-path"];
        run(runner, args);
    }

    #[test]
    fn test_print_help() {
        let mut runner = MockRunner::new();
        runner.expect_print_help().times(1).returning(|_| ());
        let args = vec!["cli"];
        run(runner, args);
    }
}
