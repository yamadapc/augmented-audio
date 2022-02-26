use audio_processor_testing_helpers::relative_path;
use iced::Command;

use audio_processor_file::AudioFileProcessor;
use audio_processor_iced_storybook::StoryView;
use audio_processor_traits::{AudioProcessorSettings, InterleavedAudioBuffer};

use super::*;

pub fn default() -> Story {
    Story::default()
}

pub struct Story {
    editor: AudioEditorView,
}

impl Default for Story {
    fn default() -> Self {
        let mut editor = AudioEditorView::default();
        let settings = AudioProcessorSettings::default();
        log::info!("Reading audio file");
        let mut audio_file_buffer = get_example_file_buffer(settings);
        let transients = audio_processor_analysis::transient_detection::stft::find_transients(
            Default::default(),
            &mut InterleavedAudioBuffer::new(1, &mut audio_file_buffer),
        );
        let markers_from_transients = {
            let mut markers = vec![];
            let mut inside_transient = false;
            for (index, sample) in transients.iter().cloned().enumerate() {
                if sample >= 0.4 && !inside_transient {
                    inside_transient = true;
                    markers.push(index);
                } else if sample < 0.4 {
                    inside_transient = false;
                }
            }
            markers
        };

        log::info!("Building editor model");
        editor.markers = markers_from_transients
            .into_iter()
            .map(|position_samples| AudioFileMarker { position_samples })
            .collect();
        editor.audio_file_model = Some(AudioFileModel::from_buffer(settings, audio_file_buffer));

        log::info!("Starting");
        Self { editor }
    }
}

fn get_example_file_buffer(settings: AudioProcessorSettings) -> Vec<f32> {
    let mut processor = AudioFileProcessor::from_path(
        audio_garbage_collector::handle(),
        settings,
        &relative_path!("../../../augmented/audio/audio-processor-analysis/hiphop-drum-loop.mp3"),
        // &relative_path!("../../../../input-files/synthetizer-loop.mp3"),
    )
    .unwrap();
    processor.prepare(settings);
    let channels = processor.buffer().clone();
    let mut output = vec![];
    for (s1, s2) in channels[0].iter().zip(channels[1].clone()) {
        output.push(s1 + s2);
    }
    output
}

impl StoryView<Message> for Story {
    fn update(&mut self, message: Message) -> Command<Message> {
        self.editor.update(message);
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        self.editor.view()
    }
}
