use augmented::gui::iced_baseview;
use augmented::gui::iced_baseview::{
    alignment, Application, Command, Container, Element, Length, Text, WindowQueue,
};
use augmented::gui::iced_editor::IcedEditor;
use augmented::vst::editor::Editor;

struct DemoApp {}

impl Application for DemoApp {
    type Executor = iced_baseview::executor::Default;
    type Message = ();
    type Flags = ();

    fn new(_: ()) -> (Self, Command<()>) {
        (Self {}, Command::none())
    }

    fn update(
        &mut self,
        _queue: &mut WindowQueue,
        _message: Self::Message,
    ) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Container::new(Text::new("Hello world"))
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}

pub fn get_editor() -> Box<dyn Editor> {
    Box::new(IcedEditor::<DemoApp>::new(()))
}
