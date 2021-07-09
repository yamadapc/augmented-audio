pub mod style {
    use iced::container::Style;
    use iced::{Background, Color};

    pub struct Container;

    impl iced::container::StyleSheet for Container {
        fn style(&self) -> Style {
            iced::container::Style {
                text_color: Some(Color::WHITE),
                background: Some(Background::Color(Color::new(0.0, 0.0, 0.0, 1.0))),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::default(),
            }
        }
    }
}
