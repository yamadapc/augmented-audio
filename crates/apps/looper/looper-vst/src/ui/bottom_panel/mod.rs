use iced::{Alignment, Button, Column, Container, Length, Row, Text};
use iced_audio::{Normal, NormalParam};
use iced_baseview::{Command, Element};

use audio_garbage_collector::Shared;
use audio_processor_iced_design_system::container::HoverContainer;
use audio_processor_iced_design_system::knob as audio_knob;
use audio_processor_iced_design_system::knob::Knob;
use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style as audio_style;
use audio_processor_iced_design_system::style::Container1;
use looper_processor::LooperProcessorHandle;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum ParameterId {
    LoopVolume,
    DryVolume,
    PlaybackSpeed,
}

pub struct ParameterViewModel {
    id: ParameterId,
    name: String,
    suffix: String,
    value: f32,
    knob_state: iced_audio::knob::State,
}

impl ParameterViewModel {
    pub fn new(id: ParameterId, name: String, suffix: String, value: f32) -> Self {
        ParameterViewModel {
            id,
            name,
            suffix,
            value,
            knob_state: iced_audio::knob::State::new(NormalParam::from(value)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    KnobChange(ParameterId, Normal),
    RecordPressed,
    ClearPressed,
    StopPressed,
}

pub struct BottomPanelView {
    processor_handle: Shared<LooperProcessorHandle>,
    parameter_states: Vec<ParameterViewModel>,
    buttons_view: ButtonsView,
}

impl BottomPanelView {
    pub fn new(processor_handle: Shared<LooperProcessorHandle>) -> Self {
        BottomPanelView {
            processor_handle: processor_handle.clone(),
            parameter_states: vec![
                ParameterViewModel::new(
                    ParameterId::LoopVolume,
                    String::from("Loop"),
                    String::from(""),
                    1.0,
                ),
                ParameterViewModel::new(
                    ParameterId::DryVolume,
                    String::from("Dry"),
                    String::from(""),
                    0.0,
                ),
                ParameterViewModel::new(
                    ParameterId::PlaybackSpeed,
                    String::from("SPEED"),
                    String::from("x"),
                    1.0,
                ),
            ],
            buttons_view: ButtonsView::new(processor_handle),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::KnobChange(id, normal) => {
                if let Some(state) = self
                    .parameter_states
                    .iter_mut()
                    .find(|param| param.id == id)
                {
                    state.value = normal.as_f32();

                    match state.id {
                        ParameterId::LoopVolume => {
                            self.processor_handle.set_loop_volume(state.value);
                        }
                        ParameterId::DryVolume => {
                            self.processor_handle.set_dry_volume(state.value);
                        }
                        ParameterId::PlaybackSpeed => {}
                    }
                }
            }
            Message::RecordPressed => {
                self.processor_handle.toggle_recording();
            }
            Message::ClearPressed => {
                self.processor_handle.clear();
            }
            Message::StopPressed => {
                self.processor_handle.toggle_playback();
            }
        }
        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
        let knobs = self
            .parameter_states
            .iter_mut()
            .map(|parameter_view_model| parameter_view(parameter_view_model))
            .collect();
        Container::new(Column::with_children(vec![Container::new(
            Row::with_children(vec![
                Container::new(self.buttons_view.view()).center_y().into(),
                Container::new(
                    Row::with_children(knobs)
                        .spacing(Spacing::base_spacing())
                        .width(Length::Fill),
                )
                .center_x()
                .center_y()
                .width(Length::Fill)
                .into(),
            ])
            .spacing(Spacing::base_spacing()),
        )
        .center_y()
        .width(Length::Fill)
        .padding(Spacing::base_spacing())
        .into()]))
        .center_y()
        .style(Container1::default())
        .into()
    }
}

struct ButtonsView {
    processor_handle: Shared<LooperProcessorHandle>,
    record_state: iced::button::State,
    clear_state: iced::button::State,
    stop_state: iced::button::State,
}

impl ButtonsView {
    pub fn new(processor_handle: Shared<LooperProcessorHandle>) -> Self {
        ButtonsView {
            processor_handle,
            record_state: iced::button::State::default(),
            clear_state: iced::button::State::default(),
            stop_state: iced::button::State::default(),
        }
    }
}

impl ButtonsView {
    pub fn view(&mut self) -> Element<Message> {
        let record_text = if self.processor_handle.is_recording() {
            "Stop recording"
        } else {
            "Record"
        };
        let stop_text = if self.processor_handle.is_playing_back() {
            "Stop"
        } else {
            "Play"
        };

        Row::with_children(vec![
            Button::new(
                &mut self.record_state,
                Text::new(record_text).size(Spacing::small_font_size()),
            )
            .on_press(Message::RecordPressed)
            .style(audio_style::Button::default())
            .into(),
            Button::new(
                &mut self.clear_state,
                Text::new("Clear").size(Spacing::small_font_size()),
            )
            .on_press(Message::ClearPressed)
            .style(audio_style::Button::default())
            .into(),
            Button::new(
                &mut self.stop_state,
                Text::new(stop_text).size(Spacing::small_font_size()),
            )
            .on_press(Message::StopPressed)
            .style(audio_style::Button::default())
            .into(),
        ])
        .into()
    }
}

fn parameter_view(parameter_view_model: &mut ParameterViewModel) -> Element<Message> {
    let parameter_id = parameter_view_model.id.clone();
    HoverContainer::new(
        Column::with_children(vec![
            Text::new(&parameter_view_model.name)
                .size(Spacing::small_font_size())
                .into(),
            Knob::new(&mut parameter_view_model.knob_state, move |value| {
                Message::KnobChange(parameter_id, value)
            })
            .size(Length::Units(Spacing::base_control_size()))
            .style(audio_knob::style::Knob)
            .into(),
            Text::new(format!(
                "{:.2}{}",
                parameter_view_model.value, parameter_view_model.suffix
            ))
            .size(Spacing::small_font_size())
            .into(),
        ])
        .align_items(Alignment::Center)
        .spacing(Spacing::small_spacing()),
    )
    .style(audio_style::HoverContainer)
    .into()
}
