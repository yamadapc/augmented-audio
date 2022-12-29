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

use std::time::Duration;

use augmented::audio::gc::Shared;
use augmented::gui::iced::Subscription;
use augmented::gui::iced_baseview::{
    alignment, widget::Container, widget::Text, window::WindowQueue, window::WindowSubs,
    Application, Command, Element, Length,
};
use augmented::gui::iced_editor::IcedEditor;
use augmented::gui::{iced, iced_baseview};
use augmented::vst;
use augmented::vst::editor::Editor;
use augmented::vst::host::Host;
use augmented::vst::plugin::HostCallback;

#[derive(Clone, Debug)]
enum AppMessage {
    UpdateTempo,
}

struct DemoApp {
    host_callback: Shared<HostCallback>,
    tempo: f64,
}

impl Application for DemoApp {
    type Executor = iced_baseview::executor::Default;
    type Message = AppMessage;
    type Flags = Shared<HostCallback>;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                host_callback: flags,
                tempo: 0.0,
            },
            Command::none(),
        )
    }

    fn update(
        &mut self,
        _queue: &mut WindowQueue,
        message: Self::Message,
    ) -> Command<Self::Message> {
        match message {
            AppMessage::UpdateTempo => {
                self.tempo = self
                    .host_callback
                    .get_time_info(
                        (vst::api::TimeInfoFlags::TEMPO_VALID
                            | vst::api::TimeInfoFlags::PPQ_POS_VALID)
                            .bits(),
                    )
                    .map(|vst_time_info| vst_time_info.tempo)
                    .unwrap_or(0.0);
            }
        }

        Command::none()
    }

    fn subscription(
        &self,
        _window_subs: &mut WindowSubs<Self::Message>,
    ) -> Subscription<Self::Message> {
        iced_baseview::time::every(Duration::from_millis(32)).map(|_| AppMessage::UpdateTempo)
    }

    fn view(&self) -> Element<Self::Message, Self::Theme> {
        Container::new(Text::new(format!("Tempo: {:.1}", self.tempo)))
            .align_x(alignment::Horizontal::Center)
            .align_y(alignment::Vertical::Center)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }

    type Theme = iced::Theme;

    fn title(&self) -> String {
        "Host tempo plugin".to_string()
    }
}

pub fn get_editor(host_callback: Shared<HostCallback>) -> Box<dyn Editor> {
    Box::new(IcedEditor::<DemoApp>::new(host_callback))
}
