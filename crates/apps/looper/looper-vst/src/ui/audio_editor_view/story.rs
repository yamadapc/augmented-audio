use audio_processor_testing_helpers::relative_path;
use iced::Command;

use audio_processor_file::AudioFileProcessor;
use audio_processor_iced_storybook::StoryView;
use audio_processor_traits::AudioProcessorSettings;

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
        let audio_file_buffer = get_example_file_buffer(settings);
        log::info!("Building editor model");
        editor.audio_file_model = Some(AudioFileModel::from_buffer(audio_file_buffer));
        log::info!("Starting");
        Self { editor }
    }
}

fn get_example_file_buffer(settings: AudioProcessorSettings) -> Vec<f32> {
    let mut processor = AudioFileProcessor::from_path(
        audio_garbage_collector::handle(),
        settings,
        &relative_path!("../../../confirmation.mp3"),
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
