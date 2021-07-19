use iced::{Container, Length, Text};

use iced_baseview::executor;
use iced_baseview::{Application, Command, Element};

pub struct LooperApplication {}

#[derive(Debug, Clone)]
pub enum Message {}

impl Application for LooperApplication {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (LooperApplication {}, Command::none())
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let content = Text::new("HELLO YOU!");
        Container::new(content)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
