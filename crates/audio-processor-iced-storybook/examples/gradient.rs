use iced::container::Style;
use iced::{
    container, Background, Color, Container, Direction, Gradient, GradientStop, Length,
    LinearGradient, Row, Text,
};

fn main() -> iced::Result {
    audio_processor_iced_storybook::builder::<()>()
        .story_fn("Gradient", || {
            Row::with_children(vec![
                Container::new(Text::new("Hey"))
                    .style(ContainerStyle)
                    .padding(50)
                    .into(),
                Container::new(Text::new("Hey"))
                    .style(ContainerStyle)
                    .padding(50)
                    .into(),
                Container::new(Text::new("Hey"))
                    .style(ContainerStyle)
                    .padding(50)
                    .into(),
                Container::new(Text::new("Hey"))
                    .style(ContainerStyle)
                    .padding(50)
                    .into(),
                Container::new(Text::new("Hey"))
                    .style(ContainerStyle)
                    .padding(50)
                    .into(),
            ])
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
        })
        .story_fn("Gradient 2 steps", || {
            Row::with_children(vec![
                Container::new(Text::new("Hey"))
                    .style(Container2StepsStyle)
                    .padding(50)
                    .height(Length::Fill)
                    .into(),
                Container::new(Text::new("Hey"))
                    .style(Container2StepsStyle)
                    .padding(50)
                    .height(Length::Fill)
                    .into(),
                Container::new(Text::new("Hey"))
                    .style(Container2StepsStyle)
                    .padding(50)
                    .height(Length::Fill)
                    .into(),
                Container::new(Text::new("Hey"))
                    .style(Container2StepsStyle)
                    .padding(50)
                    .height(Length::Fill)
                    .into(),
                Container::new(Text::new("Hey"))
                    .style(Container2StepsStyle)
                    .padding(50)
                    .height(Length::Fill)
                    .into(),
            ])
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
        })
        .run()
}

struct ContainerStyle;

impl container::StyleSheet for ContainerStyle {
    fn style(&self) -> Style {
        Style {
            text_color: None,
            background: Some(Background::Gradient(Gradient::LinearGradient(
                LinearGradient {
                    direction: Direction::Top,
                    stops: vec![
                        GradientStop {
                            percentage: 0.0,
                            color: Color::from_rgb(0.0, 0.0, 0.0),
                        },
                        GradientStop {
                            percentage: 1.0,
                            color: Color::from_rgb(0.0, 0.0, 1.0),
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

struct Container2StepsStyle;

impl container::StyleSheet for Container2StepsStyle {
    fn style(&self) -> Style {
        Style {
            text_color: None,
            background: Some(Background::Gradient(Gradient::LinearGradient(
                LinearGradient {
                    direction: Direction::Top,
                    stops: vec![
                        GradientStop {
                            percentage: 0.0,
                            color: Color::from_rgb(1.0, 0.0, 0.0),
                        },
                        GradientStop {
                            percentage: 0.5,
                            color: Color::from_rgb(0.0, 0.0, 1.0),
                        },
                        GradientStop {
                            percentage: 1.0,
                            color: Color::from_rgb(0.0, 1.0, 0.0),
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
