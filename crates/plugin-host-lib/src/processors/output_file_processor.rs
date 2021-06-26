use audio_processor_traits::{AudioProcessor, AudioProcessorSettings};
use std::{fs, io};
use symphonia::core::probe::ProbeResult;

pub struct OutputFileSettings {
    audio_file_path: String,
}

pub struct OutputAudioFileProcessor {
    audio_settings: AudioProcessorSettings,
    output_file_settings: OutputFileSettings,
    writer: Option<hound::WavWriter<io::BufWriter<fs::File>>>,
}

impl OutputAudioFileProcessor {
    pub fn from_path(audio_settings: AudioProcessorSettings, audio_file_path: &str) -> Self {
        let output_file_settings = OutputFileSettings {
            audio_file_path: audio_file_path.to_string(),
        };
        Self::new(audio_settings, output_file_settings)
    }

    pub fn new(
        audio_settings: AudioProcessorSettings,
        output_file_settings: OutputFileSettings,
    ) -> Self {
        OutputAudioFileProcessor {
            audio_settings,
            output_file_settings,
            writer: None,
        }
    }
}

impl OutputAudioFileProcessor {
    pub fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.audio_settings = settings;
        let sample_rate = settings.sample_rate() as u32;
        log::info!("Wav file will be written with sample rate: {}", sample_rate);
        let spec = hound::WavSpec {
            channels: settings.output_channels() as u16,
            sample_rate,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        self.writer = Some(
            hound::WavWriter::create(&self.output_file_settings.audio_file_path, spec).unwrap(),
        );
    }

    pub fn process(&mut self, data: &mut [f32]) {
        if let Some(mut writer) = self.writer.as_mut() {
            for sample in data {
                writer.write_sample(*sample);
            }
        }
    }
}
