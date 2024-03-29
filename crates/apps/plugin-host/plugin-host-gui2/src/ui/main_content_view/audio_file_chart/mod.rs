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

use audio_processor_traits::AudioBuffer;
use iced::{
    widget::{
        canvas::{Cursor, Geometry, Program},
        Canvas,
    },
    Element, Rectangle,
};

use augmented::gui::iced_baseview::renderer::Theme;

pub enum Message {
    #[allow(dead_code)]
    None,
}

pub struct AudioFileModel {
    #[allow(dead_code)]
    audio_file: AudioBuffer<f32>,
}

impl AudioFileModel {
    pub fn empty() -> Self {
        Self {
            audio_file: AudioBuffer::empty(),
        }
    }
}

pub struct View<'a> {
    #[allow(dead_code)]
    model: &'a AudioFileModel,
}

impl<'a> View<'a> {
    pub fn new(model: &'a AudioFileModel) -> Self {
        Self { model }
    }

    pub fn view(self) -> Element<'a, Message> {
        Canvas::new(self).into()
    }
}

impl<'a> Program<Message> for View<'a> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        vec![]
    }
}
