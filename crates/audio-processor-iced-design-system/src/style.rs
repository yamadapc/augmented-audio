pub use button::ChromelessButton;
pub use pane_grid::PaneGrid;
pub use rule::Rule;

pub mod button {
    use iced::button::Style;
    use iced::Color;

    pub struct ChromelessButton;

    impl iced::button::StyleSheet for ChromelessButton {
        fn active(&self) -> Style {
            Style {
                shadow_offset: Default::default(),
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
                text_color: Color::new(1.0, 1.0, 1.0, 0.8),
            }
        }

        fn hovered(&self) -> Style {
            Style {
                shadow_offset: Default::default(),
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
                text_color: Color::new(1.0, 1.0, 1.0, 0.8),
            }
        }

        fn pressed(&self) -> Style {
            Style {
                shadow_offset: Default::default(),
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
                text_color: Color::new(1.0, 1.0, 1.0, 0.8),
            }
        }

        fn disabled(&self) -> Style {
            Style {
                shadow_offset: Default::default(),
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
                text_color: Color::new(1.0, 1.0, 1.0, 0.8),
            }
        }
    }
}

pub mod pane_grid {
    use crate::colors::Colors;
    use iced::pane_grid::Line;
    use iced::widget::pane_grid;

    pub struct PaneGrid;

    impl pane_grid::StyleSheet for PaneGrid {
        fn picked_split(&self) -> Option<Line> {
            Option::Some(Line {
                color: Colors::active_border_color(),
                width: 2.0,
            })
        }

        fn hovered_split(&self) -> Option<Line> {
            Option::Some(Line {
                color: Colors::active_border_color(),
                width: 2.0,
            })
        }
    }
}

pub mod rule {
    use crate::colors::Colors;
    use iced::widget::rule;

    pub struct Rule;

    impl rule::StyleSheet for Rule {
        fn style(&self) -> rule::Style {
            rule::Style {
                color: Colors::border_color(),
                width: 1,
                radius: 0.0,
                fill_mode: rule::FillMode::Full,
            }
        }
    }
}
