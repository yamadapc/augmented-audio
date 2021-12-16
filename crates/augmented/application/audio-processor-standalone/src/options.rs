use clap::ArgMatches;
use std::ffi::OsString;

pub enum RenderingOptions {
    Online {
        input_file: Option<String>,
    },
    Offline {
        input_file: String,
        output_file: String,
    },
}

pub struct MidiOptions {
    pub input_file: Option<String>,
}

pub struct Options {
    midi: MidiOptions,
    rendering: RenderingOptions,
}

impl Options {
    pub fn rendering(&self) -> &RenderingOptions {
        &self.rendering
    }

    pub fn midi(&self) -> &MidiOptions {
        &self.midi
    }
}

pub fn parse_options(supports_midi: bool) -> Options {
    parse_options_from(supports_midi, &mut std::env::args_os())
}

fn parse_options_from<I, T>(supports_midi: bool, args: I) -> Options
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let app = clap::App::new("audio-processor-standalone");
    let mut app = app
        .arg(clap::Arg::from_usage(
            "-i, --input-file=[INPUT_PATH] 'An input audio file to process'",
        ))
        .arg(clap::Arg::from_usage(
            "-o, --output-file=[OUTPUT_PATH] 'If specified, will render offline into this file (WAV)'",
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
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_empty_options() {
        let options = parse_options_from::<Vec<String>, String>(false, vec![]);
        assert!(options.midi().input_file.is_none());
        assert!(matches!(
            options.rendering(),
            RenderingOptions::Online { .. }
        ));
    }

    #[test]
    fn test_parse_online_options() {
        let options = parse_options_from::<Vec<String>, String>(
            false,
            vec!["program".into(), "--input-file".into(), "test.mp3".into()],
        );
        assert!(options.midi().input_file.is_none());
        assert!(matches!(
            options.rendering(),
            RenderingOptions::Online { .. }
        ));
        match options.rendering() {
            RenderingOptions::Online { input_file } => {
                assert_eq!(input_file.as_ref().unwrap(), "test.mp3");
            }
            _ => {}
        }
    }

    #[test]
    fn test_parse_midi_options() {
        let options = parse_options_from::<Vec<String>, String>(
            true,
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
            false,
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
