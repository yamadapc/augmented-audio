use audio_processor_iced_design_system::spacing::Spacing;
use iced::{Align, Button, Column, Command, Container, Element, Length, Row, Text};

pub struct PluginContentView {
    input_file_path_button_state: iced::button::State,
    audio_plugin_path_button_state: iced::button::State,
    input_file: Option<String>,
    audio_plugin_path: Option<String>,
}

#[derive(Clone, Debug)]
pub enum Message {
    OpenInputFilePathPicker,
    OpenAudioPluginFilePathPicker,
    SetInputFile(String),
    SetAudioPlugin(String),
}

impl PluginContentView {
    pub fn new(input_file: Option<String>, audio_plugin_path: Option<String>) -> Self {
        PluginContentView {
            input_file_path_button_state: iced::button::State::new(),
            audio_plugin_path_button_state: iced::button::State::new(),
            input_file,
            audio_plugin_path,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::OpenInputFilePathPicker => {
                let result = tinyfiledialogs::open_file_dialog("Input file", "", None);
                log::info!("Got response {:?}", result);
                if let Some(path) = result {
                    return Command::perform(async move { path }, |path| {
                        Message::SetInputFile(path)
                    });
                }
            }
            Message::OpenAudioPluginFilePathPicker => {
                let result = tinyfiledialogs::open_file_dialog("Audio plugin", "", None);
                log::info!("Got response {:?}", result);
                if let Some(path) = result {
                    return Command::perform(async move { path }, |path| {
                        Message::SetAudioPlugin(path)
                    });
                }
            }
            Message::SetInputFile(path) => {
                self.input_file = Some(path);
            }
            Message::SetAudioPlugin(path) => {
                self.audio_plugin_path = Some(path);
            }
        }
        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
        Column::with_children(vec![
            Self::file_picker_with_label(
                "Input file",
                &mut self.input_file_path_button_state,
                &self.input_file,
                "Select input file",
                Message::OpenInputFilePathPicker,
            ),
            Self::file_picker_with_label(
                "Audio plugin",
                &mut self.audio_plugin_path_button_state,
                &self.audio_plugin_path,
                "Select audio plugin",
                Message::OpenAudioPluginFilePathPicker,
            ),
        ])
        .spacing(Spacing::base_spacing())
        .padding(Spacing::base_spacing())
        .into()
    }

    fn file_picker_with_label<'a>(
        label: impl Into<String>,
        state: &'a mut iced::button::State,
        option: &'a Option<String>,
        button_text: impl Into<String>,
        message: Message,
    ) -> Element<'a, Message> {
        Row::with_children(vec![
            Container::new(Text::new(label))
                .width(Length::FillPortion(2))
                .align_x(Align::End)
                .center_y()
                .padding([0, Spacing::base_spacing()])
                .into(),
            Container::new(
                Row::with_children(vec![Button::new(
                    state,
                    Text::new(match option {
                        Some(file) => file.into(),
                        None => button_text.into(),
                    }),
                )
                .on_press(message)
                .style(audio_processor_iced_design_system::style::Button)
                .into()])
                .align_items(Align::Center)
                .spacing(Spacing::base_spacing()),
            )
            .center_y()
            .width(Length::FillPortion(8))
            .into(),
        ])
        .align_items(Align::Center)
        .into()
    }
}
