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
use audio_processor_iced_design_system::charts::{LineChart, LineChartMessage};
use audio_processor_iced_design_system::spacing::Spacing;
use iced::{widget::Container, Application, Command, Element, Length, Settings, Subscription};
use std::time::Duration;

fn main() -> iced::Result {
    wisual_logger::init_from_env();
    log::info!("Initializing app");
    LineChartApp::run(Settings::default())
}

struct LineChartApp {
    line_chart: LineChart,
    offset: usize,
}

#[derive(Debug, Clone)]
enum Message {
    LineChart(LineChartMessage),
    Tick,
}

impl Application for LineChartApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = iced::theme::Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            LineChartApp {
                line_chart: LineChart::new(
                    (0..1000)
                        .map(|i| (i as f32, (i as f32 / 50.).sin()))
                        .collect(),
                ),
                offset: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Line Chart")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        self.offset += 1;
        self.line_chart.set_data(
            (0..1000)
                .map(|i| (i as f32, ((self.offset + i) as f32 / 50.).sin()))
                .collect(),
        );
        Command::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced::time::every(Duration::from_millis(16)).map(|_| Message::Tick)
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let chart = self.line_chart.element().map(Message::LineChart);
        Container::new(chart)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(Spacing::base_spacing())
            .style(audio_processor_iced_design_system::style::Container0::default())
            .into()
    }
}
