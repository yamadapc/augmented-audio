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
pub use iced_audio::Knob;

pub mod style {
    use iced_audio::graphics::knob::{Appearance, NotchShape, StyleLength};
    use iced_audio::knob::ValueArcStyle;
    use iced_style::Theme;

    use crate::colors::Colors;

    pub struct Knob;

    impl iced_audio::knob::StyleSheet for Knob {
        type Style = Theme;

        fn active(&self, _style: &Self::Style) -> Appearance {
            Appearance::Arc(iced_audio::style::knob::ArcStyle {
                width: StyleLength::Units(3.),
                empty_color: Colors::background_level0(),
                filled_color: Colors::active_border_color(),
                notch: Knob::notch(),
                cap: Default::default(),
            })
        }

        fn hovered(&self, _style: &Self::Style) -> Appearance {
            Appearance::Arc(iced_audio::style::knob::ArcStyle {
                width: StyleLength::Units(3.),
                empty_color: Colors::background_level0(),
                filled_color: Colors::active_border_color(),
                notch: Knob::notch(),
                cap: Default::default(),
            })
        }

        fn dragging(&self, _style: &Self::Style) -> Appearance {
            Appearance::Arc(iced_audio::style::knob::ArcStyle {
                width: StyleLength::Units(3.),
                empty_color: Colors::background_level0(),
                filled_color: Colors::active_border_color(),
                notch: Knob::notch(),
                cap: Default::default(),
            })
        }

        fn value_arc_style(&self, _style: &Self::Style) -> Option<ValueArcStyle> {
            Some(ValueArcStyle {
                width: 1.0,
                offset: 0.0,
                empty_color: Some(Colors::background_level0()),
                left_filled_color: Default::default(),
                right_filled_color: None,
                cap: Default::default(),
            })
        }
    }

    impl Knob {
        fn notch() -> NotchShape {
            NotchShape::Line(iced_audio::style::knob::LineNotch {
                color: Colors::background_level0(),
                width: StyleLength::Scaled(0.1),
                length: StyleLength::Scaled(0.4),
                cap: Default::default(),
                offset: StyleLength::Units(0.),
            })
        }
    }
}
