use iced::{Align, Background, Color, Column, Container, Length, Row, Rule, Text};

use audio_processor_iced_design_system::colors::Colors;
use audio_processor_iced_design_system::container::HoverContainer;
use audio_processor_iced_design_system::knob as audio_knob;
use audio_processor_iced_design_system::knob::Knob;
use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style as audio_style;
use iced_baseview::container::Style;
use iced_baseview::executor;
use iced_baseview::{Application, Command, Element};

pub struct LooperApplication {
    knobs_view: KnobsView,
}

#[derive(Debug, Clone)]
pub enum Message {
    None,
}

impl Application for LooperApplication {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            LooperApplication {
                knobs_view: KnobsView::new(),
            },
            Command::none(),
        )
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Container::new(self.knobs_view.view().map(|_| Message::None))
            .center_x()
            .center_y()
            .style(ContainerStyle)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

struct ContainerStyle;

impl iced::container::StyleSheet for ContainerStyle {
    fn style(&self) -> Style {
        Style {
            text_color: Some(Color::new(1., 1., 1., 1.)),
            background: Some(Background::Color(Colors::background_level1())),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Default::default(),
        }
    }
}

struct KnobsView {
    knob_states: Vec<iced_audio::knob::State>,
}

impl KnobsView {
    pub fn new() -> Self {
        KnobsView {
            knob_states: vec![
                iced_audio::knob::State::new(Default::default()),
                iced_audio::knob::State::new(Default::default()),
                iced_audio::knob::State::new(Default::default()),
                iced_audio::knob::State::new(Default::default()),
                iced_audio::knob::State::new(Default::default()),
                iced_audio::knob::State::new(Default::default()),
            ],
        }
    }
}

impl KnobsView {
    fn view(&mut self) -> Element<()> {
        let knobs = self
            .knob_states
            .iter_mut()
            .map(|knob_state| {
                HoverContainer::new(
                    Column::with_children(vec![
                        Text::new("Dry/Wet").size(Spacing::small_font_size()).into(),
                        Knob::new(knob_state, |_| {})
                            .size(Length::Units(Spacing::base_control_size()))
                            .style(audio_knob::style::Knob)
                            .into(),
                        Text::new("0%").size(Spacing::small_font_size()).into(),
                    ])
                    .align_items(Align::Center)
                    .spacing(Spacing::small_spacing()),
                )
                .style(audio_style::HoverContainer)
                .into()
            })
            .collect();
        Column::with_children(vec![
            Container::new(Text::new("Knobs"))
                .padding([0, 0, Spacing::base_spacing(), 0])
                .into(),
            Rule::horizontal(1).style(audio_style::Rule).into(),
            Container::new(Row::with_children(knobs).spacing(Spacing::base_spacing()))
                .width(Length::Fill)
                .center_x()
                .padding([Spacing::base_spacing(), 0])
                .into(),
            Rule::horizontal(1).style(audio_style::Rule).into(),
        ])
        .into()
    }
}
