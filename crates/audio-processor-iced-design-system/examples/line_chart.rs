use audio_processor_iced_design_system::charts::{LineChart, LineChartMessage};
use iced::{Application, Clipboard, Command, Container, Element, Length, Settings};

fn main() -> iced::Result {
    wisual_logger::init_from_env();
    log::info!("Initializing app");
    LineChartApp::run(Settings::default())
}

struct LineChartApp {
    line_chart: LineChart,
}

#[derive(Debug, Clone)]
enum Message {
    LineChart(LineChartMessage),
}

impl Application for LineChartApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            LineChartApp {
                line_chart: LineChart::new(vec![(0.0, 1.0), (1.0, 3.0), (2.0, 0.0), (3.0, 1.0)]),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Line Chart")
    }

    fn update(
        &mut self,
        _message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let chart = self
            .line_chart
            .element()
            .map(move |msg| Message::LineChart(msg));
        Container::new(chart)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(audio_processor_iced_design_system::container::style::Container)
            .into()
    }
}
