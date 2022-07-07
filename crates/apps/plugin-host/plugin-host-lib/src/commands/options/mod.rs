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
use clap::{App, ArgMatches};

#[derive(Clone)]
pub struct RunOptions {
    plugin_path: String,
    input_audio: Option<String>,
    output_audio: Option<String>,
    open_editor: bool,
    watch: bool,
    audio_host_id: Option<String>,
    output_device_id: Option<String>,
    buffer_size: Option<usize>,
    sample_rate: Option<usize>,
    input_device_id: Option<String>,
    use_default_input_device: bool,
    use_mono_input: Option<usize>,
}

impl RunOptions {
    pub fn plugin_path(&self) -> &str {
        &self.plugin_path
    }

    pub fn input_audio(&self) -> &Option<String> {
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

    pub fn input_device_id(&self) -> &Option<String> {
        &self.input_device_id
    }

    pub fn use_default_input_device(&self) -> bool {
        self.use_default_input_device
    }

    pub fn use_mono_input(&self) -> Option<usize> {
        self.use_mono_input
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
            "-i, --input=[INPUT_PATH] 'An audio file to process'",
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
        .arg(clap::Arg::from_usage(
            "--input-device-id=[INPUT_DEVICE_ID] 'Open audio input with Input device id'",
        ))
        .arg(clap::Arg::from_usage(
            "--use-default-input-device 'Open audio input with the default device'",
        ))
        .arg(clap::Arg::from_usage(
            "--use-mono-input=[CHANNEL_NUMBER] 'If specified, the input stream will be mono-ed selecting the desired channel'",
        ))
}

/// Build 'RunOptions' from Clap matches
pub fn parse_run_options(matches: &ArgMatches) -> Option<RunOptions> {
    let plugin_path = matches.value_of("plugin")?.to_string();
    let input_audio = matches.value_of("input").map(|i| i.to_string());
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
    let input_device_id = matches
        .value_of("input-device-id")
        .map(|value| value.to_string());
    let use_default_input_device = matches.is_present("use-default-input-device");
    let use_mono_input = matches
        .value_of("use-mono-input")
        .map(|s| s.parse().expect("Invalid channel number"));

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
        input_device_id,
        use_default_input_device,
        use_mono_input,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_empty_options() {
        let app = build_run_command();
        let args: Vec<&str> = vec![];
        let matches = app.get_matches_from_safe(args);
        assert!(matches.is_err());
    }

    #[test]
    fn test_parse_minimal_options() {
        let app = build_run_command();
        let args: Vec<&str> = vec!["plugin-host", "--plugin", "something.dylib"];
        let matches = app.get_matches_from_safe(args).unwrap();
        let options = parse_run_options(&matches).unwrap();
        assert_eq!(options.plugin_path(), "something.dylib");
    }

    #[test]
    fn test_parse_all_options() {
        let app = build_run_command();
        let args: Vec<&str> = vec![
            "plugin-host",
            "--plugin",
            "something.dylib",
            "--input=input.mp3",
            "--output=output.mp3",
            "--watch",
            "--editor",
            "--host-id=CoreAudio",
            "--buffer-size=64",
            "--sample-rate=1000",
            "--use-mono-input=1",
            "--input-device-id=InputDevice",
            "--output-device-id=OutputDevice",
        ];
        let matches = app.get_matches_from_safe(args).unwrap();
        let options = parse_run_options(&matches).unwrap();
        assert_eq!(options.plugin_path(), "something.dylib");
        assert_eq!(options.input_audio().as_ref().unwrap(), "input.mp3");
        assert_eq!(options.output_audio().as_ref().unwrap(), "output.mp3");
        assert_eq!(options.input_device_id().as_ref().unwrap(), "InputDevice");
        assert_eq!(options.output_device_id().as_ref().unwrap(), "OutputDevice");
        assert_eq!(options.watch(), true);
        assert_eq!(options.open_editor(), true);
        assert_eq!(options.audio_host_id().as_ref().unwrap(), "CoreAudio");
        assert_eq!(options.buffer_size().unwrap(), 64);
        assert_eq!(options.sample_rate().unwrap(), 1000);
        assert_eq!(options.use_default_input_device(), false);
        assert_eq!(options.use_mono_input(), Some(1));
    }
}
