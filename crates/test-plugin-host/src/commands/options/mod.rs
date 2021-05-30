use clap::{App, ArgMatches};

pub struct RunOptions {
    plugin_path: String,
    input_audio: String,
    open_editor: bool,
}

impl RunOptions {
    pub fn plugin_path(&self) -> &str {
        &self.plugin_path
    }

    pub fn input_audio(&self) -> &str {
        &self.input_audio
    }

    pub fn open_editor(&self) -> bool {
        self.open_editor
    }
}

/// Build RunOptions parser
pub fn build_run_command<'a, 'b>() -> App<'a, 'b> {
    clap::App::new("run")
        .about("Process audio")
        .arg(clap::Arg::from_usage(
            "-p, --plugin=<PLUGIN_PATH> 'An audio-plugin to load'",
        ))
        .arg(clap::Arg::from_usage(
            "-i, --input=<INPUT_PATH> 'An audio file to process'",
        ))
        .arg(clap::Arg::from_usage(
            "-e, --editor 'Open the editor window'",
        ))
}

/// Build 'RunOptions' from Clap matches
pub fn parse_run_options(matches: ArgMatches) -> Option<RunOptions> {
    let matches = matches.subcommand_matches("run")?;
    let plugin_path = matches.value_of("plugin")?.to_string();
    let input_audio = matches.value_of("input")?.to_string();
    let open_editor = matches.is_present("editor");

    Some(RunOptions {
        plugin_path,
        input_audio,
        open_editor,
    })
}
