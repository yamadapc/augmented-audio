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

use clap::ArgMatches;

pub enum RenderingOptions {
    Online {
        input_file: Option<String>,
        input_device: Option<String>,
        output_device: Option<String>,
    },
    Offline {
        input_file: String,
        output_file: String,
    },
}

impl RenderingOptions {
    pub fn input_device(&self) -> Option<String> {
        if let RenderingOptions::Online { input_device, .. } = &self {
            input_device.clone()
        } else {
            None
        }
    }

    pub fn output_device(&self) -> Option<String> {
        if let RenderingOptions::Online { output_device, .. } = &self {
            output_device.clone()
        } else {
            None
        }
    }
}

pub struct MidiOptions {
    pub input_file: Option<String>,
}

pub struct Options {
    midi: MidiOptions,
    rendering: RenderingOptions,
    list_hosts: bool,
    list_input_devices: bool,
    list_output_devices: bool,
}

impl Options {
    pub fn rendering(&self) -> &RenderingOptions {
        &self.rendering
    }

    pub fn midi(&self) -> &MidiOptions {
        &self.midi
    }

    pub fn list_hosts(&self) -> bool {
        self.list_hosts
    }

    pub fn list_input_devices(&self) -> bool {
        self.list_input_devices
    }

    pub fn list_output_devices(&self) -> bool {
        self.list_output_devices
    }
}

pub struct ParseOptionsParams {
    pub supports_midi: bool,
}

pub fn parse_options(params: ParseOptionsParams) -> Options {
    parse_options_from(params, std::env::args_os())
}

fn parse_options_from<I, T>(params: ParseOptionsParams, args: I) -> Options
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let supports_midi = params.supports_midi;

    let app = clap::App::new("audio-processor-standalone");
    let mut app = app
        .arg(clap::Arg::from_usage(
            "-i, --input-file=[INPUT_PATH] 'An input audio file to process'",
        ))
        .arg(clap::Arg::from_usage(
            "-o, --output-file=[OUTPUT_PATH] 'If specified, will render offline into this file (WAV)'",
        ))
        .arg(clap::Arg::from_usage(
            "--input-device=[INPUT_DEVICE] 'The input device to use'",
        ))
        .arg(clap::Arg::from_usage(
            "--output-device=[OUTPUT_DEVICE] 'The output device to use'",
        ))
        .arg(clap::Arg::from_usage(
            "--list-hosts 'List the audio hosts available'",
        ))
        .arg(clap::Arg::from_usage(
            "--list-input-devices 'List the input devices available'",
        ))
        .arg(clap::Arg::from_usage(
            "--list-output-devices 'List the output devices available'",
        ));

    if supports_midi {
        app = app
            .arg(clap::Arg::from_usage(
                "--midi-input-file=[MIDI_INPUT_FILE] 'If specified, this MIDI file will be passed through the processor'",
            ));
    }

    let matches = app.get_matches_from(args);

    let midi_options = parse_midi_options(&matches);
    let rendering = parse_rendering_options(&matches);

    Options {
        midi: midi_options,
        rendering,
        list_hosts: matches.is_present("list-hosts"),
        list_input_devices: matches.is_present("list-input-devices"),
        list_output_devices: matches.is_present("list-output-devices"),
    }
}

fn parse_midi_options(matches: &ArgMatches) -> MidiOptions {
    MidiOptions {
        input_file: matches.value_of("midi-input-file").map(|s| s.into()),
    }
}

fn parse_rendering_options(matches: &ArgMatches) -> RenderingOptions {
    if matches.is_present("output-file") {
        if !matches.is_present("input-file") {
            log::error!("Please specify `--input-file`");
            std::process::exit(1);
        }

        let input_path = matches.value_of("input-file").map(|s| s.into()).unwrap();
        let output_path = matches.value_of("output-file").map(|s| s.into()).unwrap();

        RenderingOptions::Offline {
            input_file: input_path,
            output_file: output_path,
        }
    } else {
        RenderingOptions::Online {
            input_file: matches.value_of("input-file").map(|s| s.into()),
            input_device: matches.value_of("input-device").map(|s| s.into()),
            output_device: matches.value_of("output-device").map(|s| s.into()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_empty_options() {
        let options = parse_options_from::<Vec<String>, String>(
            ParseOptionsParams {
                supports_midi: false,
            },
            vec![],
        );
        assert!(options.midi().input_file.is_none());
        assert!(matches!(
            options.rendering(),
            RenderingOptions::Online { .. }
        ));
    }

    #[test]
    fn test_parse_list_hosts() {
        let options = parse_options_from::<Vec<String>, String>(
            ParseOptionsParams {
                supports_midi: false,
            },
            vec!["program".into(), "--list-hosts".into()],
        );
        assert!(options.list_hosts());
    }

    #[test]
    fn test_parse_list_input_devices() {
        let options = parse_options_from::<Vec<String>, String>(
            ParseOptionsParams {
                supports_midi: false,
            },
            vec!["program".into(), "--list-input-devices".into()],
        );
        assert!(options.list_input_devices());
    }

    #[test]
    fn test_parse_list_output_devices() {
        let options = parse_options_from::<Vec<String>, String>(
            ParseOptionsParams {
                supports_midi: false,
            },
            vec!["program".into(), "--list-output-devices".into()],
        );
        assert!(options.list_output_devices());
    }

    #[test]
    fn test_parse_online_options() {
        let options = parse_options_from::<Vec<String>, String>(
            ParseOptionsParams {
                supports_midi: false,
            },
            vec![
                "program".into(),
                "--input-file".into(),
                "test.mp3".into(),
                "--input-device".into(),
                "Microphone".into(),
                "--output-device".into(),
                "Default".into(),
            ],
        );
        assert!(options.midi().input_file.is_none());
        assert!(matches!(
            options.rendering(),
            RenderingOptions::Online { .. }
        ));
        match options.rendering() {
            RenderingOptions::Online {
                input_file,
                input_device,
                output_device,
            } => {
                assert_eq!(input_file.as_ref().unwrap(), "test.mp3");
                assert_eq!(input_device.as_ref().unwrap(), "Microphone");
                assert_eq!(output_device.as_ref().unwrap(), "Default");
            }
            _ => {}
        }
    }

    #[test]
    fn test_parse_midi_options() {
        let options = parse_options_from::<Vec<String>, String>(
            ParseOptionsParams {
                supports_midi: true,
            },
            vec![
                "program".into(),
                "--midi-input-file".into(),
                "bach.mid".into(),
            ],
        );
        assert!(options.midi().input_file.is_some());
        assert_eq!(options.midi().input_file.as_ref().unwrap(), "bach.mid")
    }

    #[test]
    fn test_parse_offline_options() {
        let options = parse_options_from::<Vec<String>, String>(
            ParseOptionsParams {
                supports_midi: false,
            },
            vec![
                "program".into(),
                "--input-file".into(),
                "test.mp3".into(),
                "--output-file".into(),
                "test.wav".into(),
            ],
        );
        assert!(options.midi().input_file.is_none());
        assert!(matches!(
            options.rendering(),
            RenderingOptions::Offline { .. }
        ));
        match options.rendering() {
            RenderingOptions::Offline {
                input_file,
                output_file,
            } => {
                assert_eq!(input_file, "test.mp3");
                assert_eq!(output_file, "test.wav");
            }
            _ => {}
        }
    }
}
