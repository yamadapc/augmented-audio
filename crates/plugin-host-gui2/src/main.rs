use std::sync::{Arc, Mutex};

use iced::{Application, Clipboard, Command, Container, Element, Length};

use audio_processor_iced_design_system as design_system;
use plugin_host_lib::audio_io::StartError;

mod ui;

fn main() -> iced::Result {
    wisual_logger::init_from_env();
    App::run(audio_processor_iced_design_system::default_settings())
}

struct App {
    #[allow(dead_code)]
    plugin_host: Arc<Mutex<plugin_host_lib::TestPluginHost>>,
    main_content_view: ui::main_content_view::MainContentView,
    start_result: Result<(), plugin_host_lib::audio_io::StartError>,
}

#[derive(Debug, Clone)]
enum AppMessage {
    Content(ui::main_content_view::Message),
    None,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = AppMessage;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut plugin_host = plugin_host_lib::TestPluginHost::default();
        let start_result = plugin_host.start().map_err(|err| {
            log::error!("Failed to start host: {:?}", err);
            err
        });
        let plugin_host = Arc::new(Mutex::new(plugin_host));
        let main_content_view = ui::main_content_view::MainContentView::new(plugin_host.clone());

        (
            App {
                plugin_host,
                main_content_view,
                start_result,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("plugin-host")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            AppMessage::Content(message) => self.main_content_view.update(message),
            _ => {}
        }
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let content = match &self.start_result {
            Ok(_) => self
                .main_content_view
                .view()
                .map(|msg| AppMessage::Content(msg)),
            Err(err) => ui::start_error_view::StartErrorView::view(err).map(|_| AppMessage::None),
        };
        Container::new(content)
            .style(design_system::style::container::Container0)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}
