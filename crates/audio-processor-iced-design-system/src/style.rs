pub use button::Button;
pub use button::ChromelessButton;
pub use container::Container0;
pub use container::Container1;
pub use hover_container::HoverContainer;
pub use pane_grid::PaneGrid;
pub use pick_list::PickList;
pub use rule::Rule;

pub mod button {
    use iced::button::Style;
    use iced::Color;
    use iced_style::Background;

    use crate::colors::Colors;

    fn button_base_style() -> Style {
        Style {
            shadow_offset: Default::default(),
            background: Some(Background::Color(Colors::background_level0())),
            border_radius: 0.0,
            border_width: 1.0,
            border_color: Colors::border_color(),
            text_color: Colors::text(),
        }
    }

    pub struct Button;

    impl iced::button::StyleSheet for Button {
        fn active(&self) -> Style {
            button_base_style()
        }

        fn hovered(&self) -> Style {
            Style {
                background: Some(Background::Color(Colors::hover_opacity(
                    Colors::background_level0(),
                ))),
                border_color: Colors::active_border_color(),
                ..button_base_style()
            }
        }

        fn pressed(&self) -> Style {
            Style {
                background: Some(Background::Color(Colors::pressed_opacity(
                    Colors::background_level0(),
                ))),
                border_color: Colors::pressed_opacity(Colors::active_border_color()),
                text_color: Colors::hover_opacity(Colors::text()),
                ..button_base_style()
            }
        }

        fn disabled(&self) -> Style {
            button_base_style()
        }
    }

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
                text_color: Color::new(1.0, 1.0, 1.0, 0.5),
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
    use iced::pane_grid::Line;
    use iced::widget::pane_grid;

    use crate::colors::Colors;

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
    use iced::widget::rule;

    use crate::colors::Colors;

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

pub mod pick_list {
    use iced::pick_list::Style;
    use iced::widget::pick_list;
    use iced_style::Background;

    use crate::colors::Colors;

    pub struct PickList;

    impl pick_list::StyleSheet for PickList {
        fn menu(&self) -> iced_style::menu::Style {
            iced_style::menu::Style {
                text_color: Colors::text(),
                background: Background::Color(Colors::background_level0()),
                border_width: 1.0,
                border_color: Colors::border_color(),
                selected_text_color: Colors::text(),
                selected_background: Background::Color(Colors::selected_background()),
            }
        }

        fn active(&self) -> Style {
            Style {
                text_color: Colors::text(),
                placeholder_color: Default::default(),
                background: Background::Color(Colors::background_level0()),
                border_radius: 0.0,
                border_width: 1.0,
                border_color: Colors::border_color(),
                icon_size: 0.5,
            }
        }

        fn hovered(&self) -> Style {
            Style {
                text_color: Colors::text(),
                placeholder_color: Default::default(),
                background: Background::Color(Colors::hover_opacity(Colors::background_level0())),
                border_radius: 0.0,
                border_width: 1.0,
                border_color: Colors::selected_background(),
                icon_size: 0.5,
            }
        }
    }
}

pub mod container {
    use iced::container::Style;
    use iced::Background;

    use crate::colors::Colors;

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

    pub struct Container1 {
        border_width: f32,
    }

    impl Default for Container1 {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Container1 {
        pub fn new() -> Self {
            Container1 { border_width: 0.0 }
        }

        pub fn border(mut self) -> Self {
            self.border_width = 1.0;
            self
        }
    }

    impl iced::container::StyleSheet for Container1 {
        fn style(&self) -> Style {
            iced::container::Style {
                text_color: Some(Colors::text()),
                background: Some(Background::Color(Colors::background_level1())),
                border_radius: 0.0,
                border_width: self.border_width,
                border_color: Colors::border_color(),
            }
        }
    }
}

mod hover_container {
    use crate::container::hover_container::style::Style;
    use crate::container::hover_container::style::StyleSheet;

    use crate::colors::Colors;

    pub struct HoverContainer;

    impl StyleSheet for HoverContainer {
        fn style(&self) -> Style {
            Style {
                text_color: None,
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Colors::border_color(),
            }
        }

        fn hovered(&self) -> Style {
            Style {
                text_color: None,
                background: None,
                border_radius: 0.0,
                border_width: 1.0,
                border_color: Colors::border_color(),
            }
        }
    }
}

pub mod v_slider {
    use crate::colors::Colors;
    use iced_audio::graphics::v_slider::Style;
    use iced_audio::v_slider::RectStyle;

    pub struct VSlider;

    impl iced_audio::style::v_slider::StyleSheet for VSlider {
        fn active(&self) -> Style {
            Style::Rect(RectStyle {
                back_color: Colors::background_level0(),
                back_border_width: 1.0,
                back_border_radius: 0.0,
                back_border_color: Colors::border_color(),
                filled_color: Colors::active_border_color(),
                handle_color: Colors::hover_opacity(Colors::text()),
                handle_height: 5,
                handle_filled_gap: 2.0,
            })
        }

        fn hovered(&self) -> Style {
            Style::Rect(RectStyle {
                back_color: Colors::background_level0(),
                back_border_width: 1.0,
                back_border_radius: 0.0,
                back_border_color: Colors::border_color(),
                filled_color: Colors::active_border_color(),
                handle_color: Colors::text(),
                handle_height: 5,
                handle_filled_gap: 2.0,
            })
        }

        fn dragging(&self) -> Style {
            Style::Rect(RectStyle {
                back_color: Colors::background_level0(),
                back_border_width: 1.0,
                back_border_radius: 0.0,
                back_border_color: Colors::border_color(),
                filled_color: Colors::active_border_color(),
                handle_color: Colors::text(),
                handle_height: 5,
                handle_filled_gap: 2.0,
            })
        }
    }
}
