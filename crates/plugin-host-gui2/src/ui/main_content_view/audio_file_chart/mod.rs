use audio_processor_traits::audio_buffer::{OwnedAudioBuffer, VecAudioBuffer};
use iced::canvas::{Cursor, Geometry, Program};
use iced::{Canvas, Element, Rectangle};

pub enum Message {
    None,
}

pub struct AudioFileModel {
    audio_file: VecAudioBuffer<f32>,
}

impl AudioFileModel {
    pub fn empty() -> Self {
        Self {
            audio_file: VecAudioBuffer::new(),
        }
    }
}

pub struct View<'a> {
    model: &'a AudioFileModel,
}

impl<'a> View<'a> {
    pub fn new(model: &'a AudioFileModel) -> Self {
        Self { model }
    }

    pub fn view(mut self) -> Element<'a, Message> {
        Canvas::new(self).into()
    }
}

impl<'a> Program<Message> for View<'a> {
    fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry> {
        vec![]
    }
}
