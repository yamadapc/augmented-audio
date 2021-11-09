use audio_processor_iced_design_system::charts::{LineChart, LineChartMessage};
use audio_processor_iced_design_system::spacing::Spacing;
use iced::{Application, Command, Container, Element, Length, Settings, Subscription};
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

    fn view(&mut self) -> Element<'_, Self::Message> {
        let chart = self.line_chart.element().map(Message::LineChart);
        Container::new(chart)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(Spacing::base_spacing())
            .style(audio_processor_iced_design_system::style::Container0::default())
            .into()
    }
}
