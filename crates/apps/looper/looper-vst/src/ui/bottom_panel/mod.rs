use iced::{Alignment, Button, Column, Container, Length, Row, Text};
use iced_audio::{Normal, NormalParam};

use audio_garbage_collector::Shared;
use audio_processor_iced_design_system::container::HoverContainer;
use audio_processor_iced_design_system::knob as audio_knob;
use audio_processor_iced_design_system::knob::Knob;
use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style as audio_style;
use audio_processor_iced_design_system::style::Container1;
use iced_baseview::{Command, Element};
use looper_processor::LooperProcessorHandle;

static LOOP_VOLUME_ID: usize = 0;
static DRY_VOLUME_ID: usize = 1;

pub struct ParameterViewModel {
    id: usize,
    name: String,
    suffix: String,
    value: f32,
    knob_state: iced_audio::knob::State,
}

impl ParameterViewModel {
    pub fn new(id: usize, name: String, suffix: String, value: f32) -> Self {
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
    KnobChange(usize, Normal),
    RecordPressed,
    ClearPressed,
    StopPressed,
}

pub struct BottomPanelView {
    processor_handle: Shared<LooperProcessorHandle<f32>>,
    parameter_states: Vec<ParameterViewModel>,
    buttons_view: ButtonsView,
}

impl BottomPanelView {
    pub fn new(processor_handle: Shared<LooperProcessorHandle<f32>>) -> Self {
        BottomPanelView {
            processor_handle: processor_handle.clone(),
            parameter_states: vec![
                ParameterViewModel::new(
                    LOOP_VOLUME_ID,
                    String::from("Loop"),
                    String::from(""),
                    1.0,
                ),
                ParameterViewModel::new(DRY_VOLUME_ID, String::from("Dry"), String::from(""), 1.0),
            ],
            buttons_view: ButtonsView::new(processor_handle),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::KnobChange(id, normal) => {
                let state = &mut self.parameter_states[id];
                state.value = normal.as_f32();

                if state.id == LOOP_VOLUME_ID {
                    self.processor_handle.set_loop_volume(state.value);
                } else if state.id == DRY_VOLUME_ID {
                    self.processor_handle.set_dry_volume(state.value);
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
            .enumerate()
            .map(|(index, parameter_view_model)| parameter_view(index, parameter_view_model))
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
    processor_handle: Shared<LooperProcessorHandle<f32>>,
    record_state: iced::button::State,
    clear_state: iced::button::State,
    stop_state: iced::button::State,
}

impl ButtonsView {
    pub fn new(processor_handle: Shared<LooperProcessorHandle<f32>>) -> Self {
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
            Button::new(&mut self.record_state, Text::new(record_text))
                .on_press(Message::RecordPressed)
                .style(audio_style::Button)
                .into(),
            Button::new(&mut self.clear_state, Text::new("Clear"))
                .on_press(Message::ClearPressed)
                .style(audio_style::Button)
                .into(),
            Button::new(&mut self.stop_state, Text::new(stop_text))
                .on_press(Message::StopPressed)
                .style(audio_style::Button)
                .into(),
        ])
        .into()
    }
}

fn parameter_view(index: usize, parameter_view_model: &mut ParameterViewModel) -> Element<Message> {
    HoverContainer::new(
        Column::with_children(vec![
            Text::new(&parameter_view_model.name)
                .size(Spacing::small_font_size())
                .into(),
            Knob::new(&mut parameter_view_model.knob_state, move |value| {
                Message::KnobChange(index, value)
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
