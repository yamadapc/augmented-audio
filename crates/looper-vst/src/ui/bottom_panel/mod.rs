use iced::{Align, Background, Button, Color, Column, Container, Length, Row, Rule, Text};

use audio_processor_iced_design_system::colors::Colors;
use audio_processor_iced_design_system::container::HoverContainer;
use audio_processor_iced_design_system::knob as audio_knob;
use audio_processor_iced_design_system::knob::Knob;
use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style as audio_style;
use audio_processor_iced_design_system::style::Container1;
use iced_audio::Normal;
use iced_baseview::canvas::{Cursor, Geometry, Program};
use iced_baseview::container::Style;
use iced_baseview::renderer::Renderer;
use iced_baseview::{executor, Canvas, Rectangle};
use iced_baseview::{Application, Command, Element};

pub struct ParameterViewModel {
    name: String,
    suffix: String,
    value: f32,
    knob_state: iced_audio::knob::State,
}

impl ParameterViewModel {
    pub fn new() -> Self {
        let knob_state = iced_audio::knob::State::new(Default::default());

        ParameterViewModel {
            name: String::from("Dry/Wet"),
            suffix: String::from(""),
            value: 0.0,
            knob_state,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    KnobChange(usize, Normal),
    RecordPressed,
}

pub struct BottomPanelView {
    parameter_states: Vec<ParameterViewModel>,
    buttons_view: ButtonsView,
}

impl BottomPanelView {
    pub fn new() -> Self {
        BottomPanelView {
            parameter_states: vec![ParameterViewModel::new()],
            buttons_view: ButtonsView::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::KnobChange(id, normal) => {
                let state = &mut self.parameter_states[id];
                state.value = normal.as_f32();
            }
            Message::RecordPressed => {}
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
    record_state: iced::button::State,
}

impl ButtonsView {
    pub fn new() -> Self {
        ButtonsView {
            record_state: iced::button::State::default(),
        }
    }
}

impl ButtonsView {
    pub fn view(&mut self) -> Element<Message> {
        Button::new(&mut self.record_state, Text::new("Record"))
            .on_press(Message::RecordPressed)
            .style(audio_style::Button)
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
        .align_items(Align::Center)
        .spacing(Spacing::small_spacing()),
    )
    .style(audio_style::HoverContainer)
    .into()
}
