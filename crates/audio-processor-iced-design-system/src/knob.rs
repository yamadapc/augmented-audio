pub use iced_audio::Knob;

pub mod style {
    use iced_audio::graphics::knob::{NotchShape, Style, StyleLength};

    use crate::colors::Colors;

    pub struct Knob;

    impl iced_audio::knob::StyleSheet for Knob {
        fn active(&self) -> Style {
            Style::Arc(iced_audio::style::knob::ArcStyle {
                width: StyleLength::Units(4.),
                empty_color: Colors::background_level0(),
                filled_color: Colors::active_border_color(),
                notch: Knob::notch(),
                cap: Default::default(),
            })
        }

        fn hovered(&self) -> Style {
            Style::Arc(iced_audio::style::knob::ArcStyle {
                width: StyleLength::Units(4.),
                empty_color: Colors::background_level0(),
                filled_color: Colors::active_border_color(),
                notch: Knob::notch(),
                cap: Default::default(),
            })
        }

        fn dragging(&self) -> Style {
            Style::Arc(iced_audio::style::knob::ArcStyle {
                width: StyleLength::Units(4.),
                empty_color: Colors::background_level0(),
                filled_color: Colors::active_border_color(),
                notch: Knob::notch(),
                cap: Default::default(),
            })
        }
    }

    impl Knob {
        fn notch() -> NotchShape {
            iced_audio::style::knob::NotchShape::Line(iced_audio::style::knob::LineNotch {
                color: Colors::text(),
                width: StyleLength::Scaled(0.05),
                length: StyleLength::Scaled(0.2),
                cap: Default::default(),
                offset: StyleLength::Units(0.),
            })
        }
    }
}
