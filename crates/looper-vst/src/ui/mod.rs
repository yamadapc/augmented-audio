use iced::{
    Background, Color, Container, Direction, Gradient, GradientStop, Length, LinearGradient, Text,
};

use audio_processor_iced_design_system::colors::Colors;
use iced_baseview::container::Style;
use iced_baseview::executor;
use iced_baseview::{Application, Command, Element};

pub struct LooperApplication {}

#[derive(Debug, Clone)]
pub enum Message {}

impl Application for LooperApplication {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (LooperApplication {}, Command::none())
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let content = Text::new("HELLO yo!");
        Container::new(content)
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
            background: Some(Background::Gradient(Gradient::LinearGradient(
                LinearGradient {
                    direction: Direction::Bottom,
                    stops: vec![
                        GradientStop {
                            percentage: 0.0,
                            color: Colors::background_level0(),
                        },
                        GradientStop {
                            percentage: 1.0,
                            color: Colors::background_level1(),
                        },
                    ],
                },
            ))),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Default::default(),
        }
    }
}
