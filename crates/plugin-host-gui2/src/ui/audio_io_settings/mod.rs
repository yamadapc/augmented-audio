use iced::{pick_list, Align, Column, Container, Element, Length, Row, Rule, Text};

use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style::{Container0, Container1};

pub struct AudioIOSettingsView {
    audio_driver_dropdown: DropdownWithLabel,
    input_device_dropdown: DropdownWithLabel,
    output_device_dropdown: DropdownWithLabel,
}

#[derive(Default)]
pub struct DropdownState {
    pub selected_option: Option<String>,
    pub options: Vec<String>,
}

pub struct ViewModel {
    pub audio_driver_state: DropdownState,
    pub input_device_state: DropdownState,
    pub output_device_state: DropdownState,
}

#[derive(Debug, Clone)]
pub enum Message {
    AudioDriverChange(String),
    InputDeviceChange(String),
    OutputDeviceChange(String),
}

impl AudioIOSettingsView {
    pub fn new(model: ViewModel) -> Self {
        AudioIOSettingsView {
            audio_driver_dropdown: DropdownWithLabel::new(
                "Audio driver",
                model.audio_driver_state.options,
                model.audio_driver_state.selected_option,
            ),
            input_device_dropdown: DropdownWithLabel::new(
                "Input device",
                model.input_device_state.options,
                model.input_device_state.selected_option,
            ),
            output_device_dropdown: DropdownWithLabel::new(
                "Output device",
                model.output_device_state.options,
                model.output_device_state.selected_option,
            ),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::AudioDriverChange(selected) => {
                self.audio_driver_dropdown.selected_option = Some(selected);
            }
            Message::InputDeviceChange(selected) => {
                self.input_device_dropdown.selected_option = Some(selected);
            }
            Message::OutputDeviceChange(selected) => {
                self.output_device_dropdown.selected_option = Some(selected);
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let header = section_heading("Audio IO Settings");
        let content = self.content_view();
        Column::with_children(vec![header.into(), content.into()])
            .width(Length::Fill)
            .into()
    }

    pub fn content_view(&mut self) -> impl Into<Element<Message>> {
        Container::new(
            Column::with_children(vec![
                self.audio_driver_dropdown
                    .view()
                    .map(|s| Message::AudioDriverChange(s)),
                self.input_device_dropdown
                    .view()
                    .map(|s| Message::InputDeviceChange(s)),
                self.output_device_dropdown
                    .view()
                    .map(|s| Message::OutputDeviceChange(s)),
            ])
            .spacing(Spacing::base_spacing()),
        )
        .padding(Spacing::base_spacing())
        .width(Length::Fill)
        .style(Container1)
    }
}

fn section_heading<'a, T: Into<String>>(label: T) -> impl Into<Element<'a, Message>> {
    let text = Text::new(label);
    Column::with_children(vec![
        Container::new(text)
            .style(Container0)
            .padding(Spacing::base_spacing())
            .into(),
        Rule::horizontal(1)
            .style(audio_processor_iced_design_system::style::Rule)
            .into(),
    ])
}

struct DropdownWithLabel {
    pick_list_state: pick_list::State<String>,
    label: String,
    options: Vec<String>,
    selected_option: Option<String>,
}

impl DropdownWithLabel {
    pub fn new(
        label: impl Into<String>,
        options: Vec<String>,
        selected_option: Option<impl Into<String>>,
    ) -> Self {
        DropdownWithLabel {
            pick_list_state: pick_list::State::default(),
            label: label.into(),
            options,
            selected_option: selected_option.map(|s| s.into()),
        }
    }

    pub fn view(&mut self) -> Element<String> {
        Row::with_children(vec![
            Container::new(Text::new(&self.label))
                .width(Length::FillPortion(2))
                .align_x(Align::End)
                .center_y()
                .padding([0, Spacing::base_spacing()])
                .into(),
            Container::new(
                pick_list::PickList::new(
                    &mut self.pick_list_state,
                    self.options.clone(),
                    self.selected_option.clone(),
                    |option| option,
                )
                .style(audio_processor_iced_design_system::style::PickList)
                .padding(Spacing::base_spacing())
                .width(Length::Fill),
            )
            .width(Length::FillPortion(8))
            .into(),
        ])
        .width(Length::Fill)
        .align_items(Align::Center)
        .into()
    }
}
