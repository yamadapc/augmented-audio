use iced::container::Style;
use iced::{
    container, Background, Color, Container, Direction, Gradient, GradientStop, LinearGradient,
    Text,
};

fn main() -> iced::Result {
    audio_processor_iced_storybook::builder::<()>()
        .story_fn("Gradient", || {
            Container::new(Text::new("Hey"))
                .style(ContainerStyle)
                .padding(50)
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
