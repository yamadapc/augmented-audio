// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
pub use button::Button;
pub use button::ChromelessButton;
pub use container::Container0;
pub use container::Container1;
pub use hover_container::HoverContainer;
pub use pane_grid::PaneGrid;
pub use pick_list::PickList;
pub use rule::Rule;

pub mod button {
    use iced::widget::button::Appearance;
    use iced::Color;
    use iced_style::{Background, Theme};

    use crate::colors::Colors;

    pub fn button_base_style() -> Appearance {
        Appearance {
            shadow_offset: Default::default(),
            background: Some(Background::Color(Colors::background_level0())),
            border_radius: 0.0,
            border_width: 1.0,
            border_color: Colors::border_color(),
            text_color: Colors::text(),
        }
    }

    pub struct Button {
        active_style: Appearance,
        hovered_style: Appearance,
        pressed_style: Appearance,
        disabled_style: Appearance,
    }

    impl Button {
        pub fn set_active(mut self, style: Appearance) -> Self {
            self.active_style = style;
            self
        }

        pub fn set_hovered(mut self, style: Appearance) -> Self {
            self.hovered_style = style;
            self
        }

        pub fn set_pressed(mut self, style: Appearance) -> Self {
            self.pressed_style = style;
            self
        }

        pub fn disabled(mut self, style: Appearance) -> Self {
            self.disabled_style = style;
            self
        }
    }

    impl Default for Button {
        fn default() -> Self {
            Self::new(true)
        }
    }

    impl Button {
        pub fn new(bordered: bool) -> Self {
            Button {
                active_style: Appearance {
                    border_width: if bordered { 1.0 } else { 0.0 },
                    ..button_base_style()
                },
                hovered_style: Appearance {
                    background: Some(Background::Color(Colors::hover_opacity(
                        Colors::background_level0(),
                    ))),
                    border_color: Colors::active_border_color(),
                    border_width: if bordered { 1.0 } else { 0.0 },
                    ..button_base_style()
                },
                pressed_style: Appearance {
                    background: Some(Background::Color(Colors::pressed_opacity(
                        Colors::background_level0(),
                    ))),
                    border_color: Colors::pressed_opacity(Colors::active_border_color()),
                    text_color: Colors::hover_opacity(Colors::text()),
                    border_width: if bordered { 1.0 } else { 0.0 },
                    ..button_base_style()
                },
                disabled_style: Appearance {
                    border_width: if bordered { 1.0 } else { 0.0 },
                    ..button_base_style()
                },
            }
        }
    }

    impl From<Button> for iced::theme::Button {
        fn from(value: Button) -> Self {
            Self::Custom(Box::new(value))
        }
    }

    impl iced::widget::button::StyleSheet for Button {
        type Style = Theme;

        fn active(&self, _style: &Self::Style) -> Appearance {
            self.active_style
        }

        fn hovered(&self, _style: &Self::Style) -> Appearance {
            self.hovered_style
        }

        fn pressed(&self, _style: &Self::Style) -> Appearance {
            self.pressed_style
        }

        fn disabled(&self, _style: &Self::Style) -> Appearance {
            self.disabled_style
        }
    }

    pub struct ChromelessButton;

    impl From<ChromelessButton> for iced::theme::Button {
        fn from(value: ChromelessButton) -> Self {
            Self::Custom(Box::new(value))
        }
    }

    impl iced::widget::button::StyleSheet for ChromelessButton {
        type Style = Theme;

        fn active(&self, _style: &Self::Style) -> Appearance {
            Appearance {
                shadow_offset: Default::default(),
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
                text_color: Color::new(1.0, 1.0, 1.0, 0.8),
            }
        }

        fn hovered(&self, _style: &Self::Style) -> Appearance {
            Appearance {
                shadow_offset: Default::default(),
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
                text_color: Color::new(1.0, 1.0, 1.0, 0.5),
            }
        }

        fn pressed(&self, _style: &Self::Style) -> Appearance {
            Appearance {
                shadow_offset: Default::default(),
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
                text_color: Color::new(1.0, 1.0, 1.0, 0.8),
            }
        }

        fn disabled(&self, _style: &Self::Style) -> Appearance {
            Appearance {
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
    use iced::widget::pane_grid;
    use iced::widget::pane_grid::Line;
    use iced_style::Theme;

    use crate::colors::Colors;

    pub struct PaneGrid;

    impl From<PaneGrid> for iced_native::theme::PaneGrid {
        fn from(value: PaneGrid) -> Self {
            Self::Custom(Box::new(value))
        }
    }

    impl pane_grid::StyleSheet for PaneGrid {
        type Style = Theme;

        fn picked_split(&self, _style: &Self::Style) -> Option<Line> {
            Some(Line {
                color: Colors::active_border_color(),
                width: 2.0,
            })
        }

        fn hovered_split(&self, _style: &Self::Style) -> Option<Line> {
            Some(Line {
                color: Colors::active_border_color(),
                width: 2.0,
            })
        }
    }
}

pub mod rule {
    use iced::widget::rule;
    use iced_style::Theme;

    use crate::colors::Colors;

    pub struct Rule;

    impl From<Rule> for iced::theme::Rule {
        fn from(value: Rule) -> Self {
            Self::Custom(Box::new(value))
        }
    }

    impl rule::StyleSheet for Rule {
        type Style = Theme;

        fn appearance(&self, _style: &Self::Style) -> rule::Appearance {
            rule::Appearance {
                color: Colors::border_color(),
                width: 1,
                radius: 0.0,
                fill_mode: rule::FillMode::Full,
            }
        }
    }
}

pub mod pick_list {
    use iced::widget::pick_list;
    use iced::widget::pick_list::Appearance;
    use iced_style::{Background, Theme};
    use std::rc::Rc;

    use crate::colors::Colors;

    pub struct PickList;

    impl Into<iced::theme::PickList> for PickList {
        fn into(self) -> iced_style::theme::PickList {
            let t = Rc::new(self);
            // TODO: This is broken
            iced_style::theme::PickList::Custom(t.clone(), t)
        }
    }

    impl iced_style::menu::StyleSheet for PickList {
        type Style = Theme;

        fn appearance(&self, _style: &Self::Style) -> iced_style::menu::Appearance {
            iced_style::menu::Appearance {
                text_color: Colors::text(),
                background: Background::Color(Colors::background_level0()),
                border_width: 1.0,
                border_radius: 0.0,
                border_color: Colors::border_color(),
                selected_text_color: Colors::text(),
                selected_background: Background::Color(Colors::selected_background()),
            }
        }
    }

    impl pick_list::StyleSheet for PickList {
        type Style = Theme;

        fn active(&self, _style: &Self::Style) -> Appearance {
            Appearance {
                text_color: Colors::text(),
                placeholder_color: Default::default(),
                background: Background::Color(Colors::background_level0()),
                border_radius: 0.0,
                border_width: 1.0,
                border_color: Colors::border_color(),
                icon_size: 0.5,
            }
        }

        fn hovered(&self, _style: &Self::Style) -> Appearance {
            Appearance {
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
    use iced::widget::container::Appearance;
    use iced::Background;
    use iced_style::Theme;

    use crate::colors::Colors;

    pub struct Container0 {
        border_width: f32,
        border_radius: f32,
    }

    impl Default for Container0 {
        fn default() -> Self {
            Self {
                border_width: 0.0,
                border_radius: 0.0,
            }
        }
    }

    impl Container0 {
        pub fn border_width(mut self, border_width: f32) -> Self {
            self.border_width = border_width;
            self
        }

        pub fn border_radius(mut self, border_radius: f32) -> Self {
            self.border_radius = border_radius;
            self
        }
    }

    impl From<Container0> for iced::theme::Container {
        fn from(value: Container0) -> Self {
            Self::Custom(Box::new(value))
        }
    }

    impl iced::widget::container::StyleSheet for Container0 {
        type Style = Theme;

        fn appearance(&self, _style: &Self::Style) -> Appearance {
            Appearance {
                text_color: Some(Colors::text()),
                background: Some(Background::Color(Colors::background_level0())),
                border_radius: self.border_radius,
                border_width: self.border_width,
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

    impl From<Container1> for iced::theme::Container {
        fn from(value: Container1) -> Self {
            Self::Custom(Box::new(value))
        }
    }

    impl iced::widget::container::StyleSheet for Container1 {
        type Style = Theme;

        fn appearance(&self, _style: &Self::Style) -> Appearance {
            Appearance {
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
    use crate::colors::Colors;
    use crate::container::hover_container::style::Style;
    use crate::container::hover_container::style::StyleSheet;

    #[derive(Default)]
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
    use iced_audio::v_slider::{Appearance, RectStyle};
    use iced_style::Theme;

    use crate::colors::Colors;

    pub struct VSlider;

    impl iced_audio::style::v_slider::StyleSheet for VSlider {
        type Style = Theme;

        fn active(&self, _style: &Self::Style) -> Appearance {
            Appearance::Rect(RectStyle {
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

        fn hovered(&self, _style: &Self::Style) -> Appearance {
            Appearance::Rect(RectStyle {
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

        fn dragging(&self, _style: &Self::Style) -> Appearance {
            Appearance::Rect(RectStyle {
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
