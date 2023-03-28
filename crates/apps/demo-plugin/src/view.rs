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
