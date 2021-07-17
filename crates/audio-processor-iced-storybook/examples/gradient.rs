use iced::container::Style;
use iced::{
    container, Background, Color, Container, Direction, Gradient, GradientStop, Length,
    LinearGradient, Row, Text,
};

fn main() -> iced::Result {
    audio_processor_iced_storybook::builder::<()>()
        .story_fn("Top to bottom", || {
            Container::new(Text::new("Top to bottom"))
                .style(TopToBottom)
                .padding(50)
                .center_x()
                .center_y()
                .height(Length::Fill)
                .width(Length::Fill)
                .into()
        })
        .story_fn("Top to bottom small", || {
            Container::new(
                Container::new(Text::new("Top to bottom"))
                    .style(TopToBottom)
                    .center_x()
                    .center_y()
                    .padding(50),
            )
            .height(Length::Fill)
            .width(Length::Fill)
            .center_x()
            .center_y()
            .into()
        })
        .story_fn("Left to right", || {
            Container::new(Text::new("Left to right"))
                .style(LeftToRight)
                .center_x()
                .center_y()
                .padding(50)
                .height(Length::Fill)
                .width(Length::Fill)
                .into()
        })
        .story_fn("Left to right small", || {
            Container::new(
                Container::new(Text::new("Left to right"))
                    .style(LeftToRight)
                    .center_x()
                    .center_y()
                    .padding(50),
            )
            .height(Length::Fill)
            .width(Length::Fill)
            .center_x()
            .center_y()
            .into()
        })
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
        .story_fn("Red to blue", || {
            Row::with_children(vec![Container::new(Text::new("Red -> Blue"))
                .style(StyleHolder(Style {
                    text_color: None,
                    background: Some(Background::Gradient(Gradient::LinearGradient(
                        LinearGradient {
                            direction: Direction::Right,
                            stops: vec![
                                GradientStop {
                                    percentage: 0.0,
                                    color: Color::from_rgb(1.0, 0.0, 0.0),
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
                }))
                .padding(50)
                .height(Length::Fill)
                .width(Length::Fill)
                .into()])
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
        })
        .story_fn("Pretty", || {
            Row::with_children(vec![Container::new(Text::new("Hey"))
                .style(ContainerPrettyStyle)
                .padding(50)
                .height(Length::Fill)
                .width(Length::Fill)
                .into()])
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
        })
        .story_fn("Gradient & Border", || {
            Container::new(
                Container::new(Text::new("Gradient & border"))
                    .style(StyleHolder(Style {
                        text_color: None,
                        background: Some(Background::Gradient(Gradient::LinearGradient(
                            LinearGradient {
                                direction: Direction::Bottom,
                                stops: vec![
                                    GradientStop {
                                        percentage: 0.0,
                                        color: Color::from_rgb(63. / 255., 64. / 255., 73. / 255.),
                                    },
                                    GradientStop {
                                        percentage: 1.0,
                                        color: Color::from_rgb(33. / 255., 34. / 255., 43. / 255.)
                                            .darken(0.9),
                                    },
                                ],
                            },
                        ))),
                        border_radius: 5.0,
                        border_width: 0.0,
                        border_color: Color::new(1.0, 0.0, 0.0, 1.),
                    }))
                    .padding(50),
            )
            .height(Length::Fill)
            .center_y()
            .style(StyleHolder(Style {
                text_color: None,
                background: Some(Background::Color(Color::new(1., 1., 1., 1.))),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
            }))
            .center_x()
            .width(Length::Fill)
            .into()
        })
        .run()
}

struct TopToBottom;

impl container::StyleSheet for TopToBottom {
    fn style(&self) -> Style {
        Style {
            text_color: None,
            background: Some(Background::Gradient(Gradient::LinearGradient(
                LinearGradient {
                    direction: Direction::Bottom,
                    stops: vec![
                        GradientStop {
                            percentage: 0.0,
                            color: Color::from_rgb(0.0, 1.0, 0.0),
                        },
                        GradientStop {
                            percentage: 0.5,
                            color: Color::from_rgb(1.0, 0.0, 0.0),
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

struct LeftToRight;

impl container::StyleSheet for LeftToRight {
    fn style(&self) -> Style {
        Style {
            text_color: None,
            background: Some(Background::Gradient(Gradient::LinearGradient(
                LinearGradient {
                    direction: Direction::Right,
                    stops: vec![
                        GradientStop {
                            percentage: 0.0,
                            color: Color::from_rgb(0.0, 1.0, 0.0),
                        },
                        GradientStop {
                            percentage: 0.5,
                            color: Color::from_rgb(1.0, 0.0, 0.0),
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

struct ContainerStyle;

impl container::StyleSheet for ContainerStyle {
    fn style(&self) -> Style {
        Style {
            text_color: None,
            background: Some(Background::Gradient(Gradient::LinearGradient(
                LinearGradient {
                    direction: Direction::Bottom,
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

struct StyleHolder(Style);
impl container::StyleSheet for StyleHolder {
    fn style(&self) -> Style {
        self.0.clone()
    }
}

struct Container2StepsStyle;

impl container::StyleSheet for Container2StepsStyle {
    fn style(&self) -> Style {
        Style {
            text_color: None,
            background: Some(Background::Gradient(Gradient::LinearGradient(
                LinearGradient {
                    direction: Direction::Bottom,
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

struct ContainerPrettyStyle;

impl container::StyleSheet for ContainerPrettyStyle {
    fn style(&self) -> Style {
        Style {
            text_color: None,
            background: Some(Background::Gradient(Gradient::LinearGradient(
                LinearGradient {
                    direction: Direction::Bottom,
                    stops: vec![
                        GradientStop {
                            percentage: 0.0,
                            color: Color::from_rgb(33. / 255., 34. / 255., 43. / 255.),
                        },
                        GradientStop {
                            percentage: 0.25,
                            color: Color::from_rgb(33. / 255., 34. / 255., 43. / 255.).darken(0.3),
                        },
                        GradientStop {
                            percentage: 0.75,
                            color: Color::from_rgb(33. / 255., 34. / 255., 43. / 255.).darken(0.4),
                        },
                        GradientStop {
                            percentage: 1.0,
                            color: Color::from_rgb(33. / 255., 34. / 255., 43. / 255.).darken(0.7),
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
