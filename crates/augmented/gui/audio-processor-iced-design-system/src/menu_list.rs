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
use crate::spacing::Spacing;
use iced::{widget::Button, widget::Column, Element, Length};

pub struct State<InnerState, MenuOption> {
    selected_child: Option<usize>,
    children: Vec<menu_item::State<InnerState, MenuOption>>,
}

#[derive(Debug, Clone)]
pub enum Message<MenuOption> {
    Selected { index: usize, option: MenuOption },
}

impl<InnerState, MenuOption: Clone> State<InnerState, MenuOption> {
    pub fn new(children: Vec<(InnerState, MenuOption)>, selected_child: Option<usize>) -> Self {
        State {
            selected_child,
            children: children
                .into_iter()
                .map(|(child, option)| menu_item::State::new(child, option))
                .collect(),
        }
    }

    pub fn update(&mut self, message: Message<MenuOption>) {
        match message {
            Message::Selected { index, .. } => {
                self.selected_child = Some(index);
            }
        }
    }

    pub fn view(
        &self,
        renderer: impl Fn(&InnerState) -> Element<Message<MenuOption>>,
    ) -> Element<Message<MenuOption>> {
        let selected_child = self.selected_child;
        let children_elements = self
            .children
            .iter()
            .enumerate()
            .map(|(index, menu_item::State { state, option })| {
                let is_selected_child =
                    selected_child.is_some() && selected_child.unwrap() == index;
                let inner = renderer(state);
                Button::new(inner)
                    .width(Length::Fill)
                    .style(style::ButtonStyleSheet(is_selected_child).into())
                    .padding(Spacing::base_spacing())
                    .on_press(Message::Selected {
                        index,
                        option: option.clone(),
                    })
                    .into()
            })
            .collect();

        Column::with_children(children_elements)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

mod menu_item {
    pub struct State<InnerState, MenuOption> {
        pub state: InnerState,
        pub option: MenuOption,
    }

    impl<InnerState, MenuOption> State<InnerState, MenuOption> {
        pub fn new(state: InnerState, option: MenuOption) -> Self {
            State { state, option }
        }
    }
}

mod style {
    use crate::colors::Colors;
    use iced::widget::button::Appearance;
    use iced::Background;
    use iced_style::Theme;

    pub struct ButtonStyleSheet(pub bool);

    impl From<ButtonStyleSheet> for iced::theme::Button {
        fn from(value: ButtonStyleSheet) -> Self {
            Self::Custom(Box::new(value))
        }
    }

    impl iced::widget::button::StyleSheet for ButtonStyleSheet {
        type Style = Theme;

        fn active(&self, _style: &Self::Style) -> Appearance {
            Appearance {
                shadow_offset: Default::default(),
                background: if self.0 {
                    Some(Background::Color(Colors::selected_background()))
                } else {
                    None
                },
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
                text_color: Colors::text(),
            }
        }

        fn hovered(&self, _style: &Self::Style) -> Appearance {
            Appearance {
                shadow_offset: Default::default(),
                background: if self.0 {
                    Some(Background::Color(Colors::selected_background()))
                } else {
                    Some(Background::Color(Colors::background_level1()))
                },
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
                text_color: Colors::text(),
            }
        }

        fn pressed(&self, _style: &Self::Style) -> Appearance {
            Appearance {
                shadow_offset: Default::default(),
                background: if self.0 {
                    Some(Background::Color(Colors::selected_background()))
                } else {
                    Some(Background::Color(Colors::background_level2()))
                },
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
                text_color: Colors::text(),
            }
        }

        fn disabled(&self, _style: &Self::Style) -> Appearance {
            Appearance {
                shadow_offset: Default::default(),
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Default::default(),
                text_color: Colors::text(),
            }
        }
    }
}
