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

use derive_more::From;

use audio_processor_iced_design_system as design_system;
use augmented::gui::iced::{Application, Command, Container, Element, Length, Subscription};
use plugin_host_lib::actor_system::ActorSystemThread;
use plugin_host_lib::audio_io::audio_thread::options::AudioThreadOptions;
use plugin_host_lib::audio_io::audio_thread::AudioThread;
use plugin_host_lib::TestPluginHost;
use ui::main_content_view;

pub mod executor;
mod ops;
pub mod services;
pub mod ui;
mod utils;

pub struct App {
    #[allow(unused)]
    actor_system_thread: &'static ActorSystemThread,
    main_content_view: main_content_view::MainContentView,
    start_result: Result<(), plugin_host_lib::audio_io::StartError>,
}

#[derive(Debug, Clone, From)]
pub enum AppMessage {
    Content(main_content_view::Message),
    OpenGithub,
    None,
}

impl Application for App {
    type Executor = executor::PluginHostExecutor;
    type Message = AppMessage;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let version = utils::get_version();
        log::info!(
            "plugin-host-gui2: Application is booting - VERSION={}",
            version
        );

        let actor_system_thread = ActorSystemThread::current();

        let mut plugin_host = {
            let audio_settings = AudioThread::default_settings().unwrap();
            let audio_thread_options = AudioThreadOptions::default();
            actor_system_thread.spawn_result(async move {
                TestPluginHost::new(audio_settings, audio_thread_options, true)
            })
        };
        let start_result = plugin_host.start_audio().map_err(|err| {
            log::error!("Failed to start host: {:?}", err);
            err
        });
        let (main_content_view, command) =
            main_content_view::MainContentView::new(plugin_host, actor_system_thread);

        log::info!("plugin-host-gui2: Start-up is complete");
        (
            App {
                main_content_view,
                actor_system_thread,
                start_result,
            },
            command.map(|msg| msg.into()),
        )
    }

    fn title(&self) -> String {
        String::from("plugin-host")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            AppMessage::Content(message) => self
                .main_content_view
                .update(message)
                .map(AppMessage::Content),
            AppMessage::OpenGithub => {
                if let Err(err) = opener::open("https://github.com/yamadapc/augmented-audio") {
                    log::error!("Failed to open GitHub page: {}", err);
                }
                Command::none()
            }
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
            .style(design_system::style::container::Container0::default())
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }

    // fn menu(&self) -> Menu<Self::Message> {
    //     iced::menu::Menu::with_entries(vec![
    //         iced::menu::Entry::Dropdown {
    //             title: "File".to_string(),
    //             submenu: iced::menu::Menu::with_entries(vec![iced::menu::Entry::Item {
    //                 on_activation: AppMessage::Content(main_content_view::Message::PluginContent(
    //                     main_content_view::plugin_content::Message::OpenAudioPluginFilePathPicker,
    //                 )),
    //                 hotkey: Some(iced_core::keyboard::Hotkey {
    //                     key: iced_core::keyboard::KeyCode::O,
    //                     modifiers: iced::keyboard::Modifiers::LOGO,
    //                 }),
    //                 title: "Open plugin...".to_string(),
    //             }]),
    //         },
    //         iced::menu::Entry::Dropdown {
    //             title: "Help".to_string(),
    //             submenu: iced::menu::Menu::with_entries(vec![iced::menu::Entry::Item {
    //                 on_activation: AppMessage::OpenGithub,
    //                 hotkey: None,
    //                 title: "Open GitHub project".to_string(),
    //             }]),
    //         },
    //     ])
    // }
}
