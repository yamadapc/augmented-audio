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
use iced::Color;

fn dark_blue() -> Color {
    rgb(35, 136, 201)
}

fn white() -> Color {
    rgb(255, 255, 255)
}

pub fn black() -> Color {
    rgb(19, 19, 19)
}

fn medium_gray() -> Color {
    rgb(42, 42, 42)
}

fn gray() -> Color {
    rgb(50, 50, 50)
}

pub fn light_gray() -> Color {
    rgb(60, 60, 60)
}

fn super_light_gray() -> Color {
    rgb(118, 118, 118)
}

pub fn green() -> Color {
    rgb(73, 190, 84)
}

pub fn red() -> Color {
    rgb(199, 84, 80)
}

pub fn yellow() -> Color {
    rgb(240, 187, 104)
}

pub struct Colors;

impl Colors {
    pub fn text() -> Color {
        white()
    }

    pub fn success() -> Color {
        green()
    }

    pub fn error() -> Color {
        red()
    }

    pub fn warning() -> Color {
        yellow()
    }

    pub fn idle() -> Color {
        Self::background_level0()
    }

    pub fn background_level0() -> Color {
        black()
    }

    pub fn hover_opacity(color: Color) -> Color {
        Color::new(color.r, color.g, color.b, color.a * 0.5)
    }

    pub fn pressed_opacity(color: Color) -> Color {
        Color::new(color.r, color.g, color.b, color.a * 0.4)
    }

    pub fn background_level1() -> Color {
        medium_gray()
    }

    pub fn background_level2() -> Color {
        gray()
    }

    pub fn border_color() -> Color {
        // super_light_gray.darken(0.3)
        Color::BLACK
    }

    pub fn selected_background() -> Color {
        dark_blue()
    }

    pub fn active_border_color() -> Color {
        dark_blue()
    }
}

fn rgb(r: i32, g: i32, b: i32) -> Color {
    Color::new(r as f32 / 255., g as f32 / 255., b as f32 / 255., 1.)
}
