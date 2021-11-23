pub enum Options {
    OnlineRendering {
        input_path: Option<String>,
    },
    OfflineRendering {
        input_path: String,
        output_path: String,
    },
}

pub fn parse_options() -> Options {
    let app = clap::App::new("audio-processor-standalone");
    let matches = app
        .arg(clap::Arg::from_usage(
            "-i, --input=[INPUT_PATH] 'An input audio file to process'",
        ))
        .arg(clap::Arg::from_usage(
            "-o, --output=[OUTPUT_PATH] 'If specified, will render offline into this file (WAV)'",
        ))
        .get_matches();

    if matches.is_present("output") {
        if !matches.is_present("input") {
            log::error!("Please specify `--input`");
            std::process::exit(1);
        }

        let input_path = matches.value_of("input").map(|s| s.into()).unwrap();
        let output_path = matches.value_of("output").map(|s| s.into()).unwrap();

        Options::OfflineRendering {
            input_path,
            output_path,
        }
    } else {
        Options::OnlineRendering {
            input_path: matches.value_of("input").map(|s| s.into()),
        }
    }
}
