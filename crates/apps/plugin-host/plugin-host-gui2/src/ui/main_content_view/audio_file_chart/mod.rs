use audio_processor_traits::audio_buffer::{OwnedAudioBuffer, VecAudioBuffer};
use iced::canvas::{Cursor, Geometry, Program};
use iced::{Canvas, Element, Rectangle};

pub enum Message {
    #[allow(dead_code)]
    None,
}

pub struct AudioFileModel {
    #[allow(dead_code)]
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
    fn draw(&self, _bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        vec![]
    }
}
