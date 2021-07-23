use std::time::Duration;

use derive_more::From;
use iced::{Application, Clipboard, Command, Container, Element, Length, Subscription};

use audio_processor_iced_design_system as design_system;
use ui::main_content_view;

pub mod services;
pub mod ui;

pub struct App {
    main_content_view: main_content_view::MainContentView,
    start_result: Result<(), plugin_host_lib::audio_io::StartError>,
}

#[derive(Debug, Clone, From)]
pub enum AppMessage {
    Content(main_content_view::Message),
    None,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = AppMessage;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        log::info!("plugin-host-gui2: Application is booting");
        let mut plugin_host = plugin_host_lib::TestPluginHost::default();
        let start_result = plugin_host.start().map_err(|err| {
            log::error!("Failed to start host: {:?}", err);
            err
        });
        let (main_content_view, command) = main_content_view::MainContentView::new(plugin_host);

        (
            App {
                main_content_view,
                start_result,
            },
            command.map(|msg| msg.into()),
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
            AppMessage::Content(message) => self
                .main_content_view
                .update(message)
                .map(AppMessage::Content),
            _ => self
                .main_content_view
                .update(main_content_view::Message::None)
                .map(AppMessage::Content),
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let mut subscriptions =
            vec![iced::time::every(Duration::from_millis(16)).map(|_| AppMessage::None)];
        subscriptions.push(
            self.main_content_view
                .subscription()
                .map(AppMessage::Content),
        );
        Subscription::batch(subscriptions)
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let content = match &self.start_result {
            Ok(_) => self.main_content_view.view().map(AppMessage::Content),
            Err(err) => ui::start_error_view::StartErrorView::view(err).map(|_| AppMessage::None),
        };
        Container::new(content)
            .style(design_system::style::container::Container0)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }
}
