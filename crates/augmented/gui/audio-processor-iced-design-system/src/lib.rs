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
pub mod charts;
pub mod colors;
pub mod container;
pub mod dropdown_with_label;
pub mod knob;
pub mod menu_list;
pub mod router;
pub mod spacing;
pub mod style;
pub mod tabs;
pub mod tree_view;
pub mod updatable;

pub fn default_settings<Flags: Default>() -> iced::Settings<Flags> {
    iced::Settings {
        antialiasing: true,
        default_text_size: spacing::Spacing::default_font_size() as f32,
        window: iced::window::Settings {
            size: (1400, 1024),
            ..iced::window::Settings::default()
        },
        ..iced::Settings::default()
    }
}
