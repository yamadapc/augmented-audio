// Augmented Audio: Audio libraries and applications
use augmented::gui::iced;
use augmented::gui::iced_baseview;
use augmented::gui::iced_baseview::alignment;
use augmented::gui::iced_baseview::widget::Container;
use augmented::gui::iced_baseview::widget::Text;
use augmented::gui::iced_baseview::window::WindowQueue;
use augmented::gui::iced_baseview::Application;
use augmented::gui::iced_baseview::Command;
use augmented::gui::iced_baseview::Element;
use augmented::gui::iced_baseview::Length;
use augmented::gui::iced_editor::IcedEditor;
use augmented::vst::editor::Editor;

struct DemoApp {}

impl Application for DemoApp {
    type Executor = iced_baseview::executor::Default;
    type Message = ();
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_: ()) -> (Self, Command<()>) {
        (Self {}, Command::none())
    }

    fn title(&self) -> String {
        "Demo plugin".into()
    }

    fn update(
        &mut self,
        _queue: &mut WindowQueue,
        _message: Self::Message,
    ) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message, Self::Theme> {
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
