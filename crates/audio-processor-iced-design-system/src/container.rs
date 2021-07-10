pub mod style {
    use crate::colors::Colors;
    use iced::container::Style;
    use iced::Background;

    pub struct Container0;

    impl iced::container::StyleSheet for Container0 {
        fn style(&self) -> Style {
            iced::container::Style {
                text_color: Some(Colors::text()),
                background: Some(Background::Color(Colors::background_level0())),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Colors::border_color(),
            }
        }
    }

    pub struct Container1;

    impl iced::container::StyleSheet for Container1 {
        fn style(&self) -> Style {
            iced::container::Style {
                text_color: Some(Colors::text()),
                background: Some(Background::Color(Colors::background_level1())),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Colors::border_color(),
            }
        }
    }
}
