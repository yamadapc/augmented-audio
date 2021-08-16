use audio_processor_iced_design_system::colors::Colors;
use iced_baseview::container::Style;
use iced_baseview::{Background, Color};

pub struct ContainerStyle;

impl iced::container::StyleSheet for ContainerStyle {
    fn style(&self) -> Style {
        Style {
            text_color: Some(Color::new(1., 1., 1., 1.)),
            background: Some(Background::Color(Colors::background_level0())),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Default::default(),
        }
    }
}
