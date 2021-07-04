use clap::{App, ArgMatches};

#[derive(Clone)]
pub struct RunOptions {
    plugin_path: String,
    input_audio: String,
    output_audio: Option<String>,
    open_editor: bool,
    watch: bool,
    audio_host_id: Option<String>,
    output_device_id: Option<String>,
    buffer_size: Option<usize>,
    sample_rate: Option<usize>,
}

impl RunOptions {
    pub fn plugin_path(&self) -> &str {
        &self.plugin_path
    }

    pub fn input_audio(&self) -> &str {
        &self.input_audio
    }

    pub fn output_audio(&self) -> &Option<String> {
        &self.output_audio
    }

    pub fn open_editor(&self) -> bool {
        self.open_editor
    }

    pub fn watch(&self) -> bool {
        self.watch
    }

    pub fn audio_host_id(&self) -> &Option<String> {
        &self.audio_host_id
    }

    pub fn output_device_id(&self) -> &Option<String> {
        &self.output_device_id
    }

    pub fn buffer_size(&self) -> Option<usize> {
        self.buffer_size
    }

    pub fn sample_rate(&self) -> Option<usize> {
        self.sample_rate
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
            "-o, --output=[OUTPUT_PATH] 'If specified, will render offline into file'",
        ))
        .arg(clap::Arg::from_usage(
            "-e, --editor 'Open the editor window'",
        ))
        .arg(clap::Arg::from_usage(
            "-w, --watch 'Watch and reload the VST when it changes'",
        ))
        .arg(clap::Arg::from_usage(
            "--host-id=[HOST_ID] 'Audio host name'",
        ))
        .arg(clap::Arg::from_usage(
            "--output-device-id=[OUTPUT_DEVICE_ID] 'Output device id'",
        ))
        .arg(clap::Arg::from_usage(
            "--buffer-size=[BUFFER_SIZE] 'Buffer size'",
        ))
        .arg(clap::Arg::from_usage(
            "--sample-rate=[SAMPLE_RATE] 'Sample rate'",
        ))
}

/// Build 'RunOptions' from Clap matches
pub fn parse_run_options(matches: ArgMatches) -> Option<RunOptions> {
    let matches = matches.subcommand_matches("run")?;
    let plugin_path = matches.value_of("plugin")?.to_string();
    let input_audio = matches.value_of("input")?.to_string();
    let output_audio = matches.value_of("output").map(|value| value.to_string());
    let open_editor = matches.is_present("editor");
    let watch = matches.is_present("watch");

    // Audio thread options
    let audio_host_id = matches.value_of("host-id").map(|value| value.to_string());
    let output_device_id = matches
        .value_of("output-device-id")
        .map(|value| value.to_string());
    let buffer_size = matches
        .value_of("buffer-size")
        .map(|value| value.parse().expect("Invalid buffer size"));
    let sample_rate = matches
        .value_of("sample-rate")
        .map(|value| value.parse().expect("Invalid sample rate"));

    Some(RunOptions {
        plugin_path,
        input_audio,
        output_audio,
        open_editor,
        watch,
        audio_host_id,
        output_device_id,
        buffer_size,
        sample_rate,
    })
}
