use iced::{Background, Color};

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
use augmented::gui::iced_baseview::widget::container::Appearance;

pub struct ContainerStylesheet {
    pub text_color: Option<Color>,
    pub background: Option<Background>,
    pub border_radius: f32,
    pub border_width: f32,
    pub border_color: Color,
}

impl ContainerStylesheet {
    #[allow(dead_code)]
    pub fn with_text_color(mut self, color: Option<Color>) -> Self {
        self.text_color = color;
        self
    }

    #[allow(dead_code)]
    pub fn with_background(mut self, background: Option<Background>) -> Self {
        self.background = background;
        self
    }

    #[allow(dead_code)]
    pub fn with_border_radius(mut self, border_radius: f32) -> Self {
        self.border_radius = border_radius;
        self
    }

    #[allow(dead_code)]
    pub fn with_border_width(mut self, border_width: f32) -> Self {
        self.border_width = border_width;
        self
    }

    #[allow(dead_code)]
    pub fn with_border_color(mut self, border_color: Color) -> Self {
        self.border_color = border_color;
        self
    }
}

impl Default for ContainerStylesheet {
    fn default() -> Self {
        iced::widget::container::Appearance::default().into()
    }
}

impl From<iced::widget::container::Appearance> for ContainerStylesheet {
    fn from(style: iced::widget::container::Appearance) -> Self {
        Self {
            text_color: style.text_color,
            background: style.background,
            border_radius: style.border_radius,
            border_width: style.border_width,
            border_color: style.border_color,
        }
    }
}

impl iced::widget::container::StyleSheet for ContainerStylesheet {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        Appearance {
            text_color: self.text_color,
            background: self.background,
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color,
        }
    }
}
