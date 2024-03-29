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

//!
//! This is a very simple draft of a "storybook" style library for iced.
//!
//! It's similar in spirit to ["Storybook for react"](https://storybook.js.org/docs/react/get-started/introduction).
//!
//! ![](https://raw.githubusercontent.com/yamadapc/augmented-audio/master/crates/augmented/gui/audio-processor-iced-storybook/screenshot.png)
//!
//! As of current it simply:
//! * Shows a sidebar with story names
//! * Shows stories in the content view when they're clicked
//!
//! ## Usage
//! Stories are configured using the `builder` function. They're registered by name in **two** possible ways.
//!
//! The first way is to declare the story as a `story_fn`, which is just for rendering stateless elements:
//! ```ignore
//! use iced::{Text, Length, Container};
//!
//! use audio_processor_iced_storybook as storybook;
//!
//! type Message = ();
//!
//! fn main() {
//!     storybook::builder::<Message>()
//!         .story_fn("Hello world", || {
//!             Container::new(Text::new("Hey!"))
//!                 .padding(50)
//!                 .center_x()
//!                 .center_y()
//!                 .height(Length::Fill)
//!                 .width(Length::Fill)
//!                 .into()
//!         })
//!         .run()
//!         .unwrap();
//! }
//! ```
//! `storybook::builder` takes the children message type as a type-parameter. It's required that this type conforms to
//! `'static + Clone + Debug`.
//!
//! Stories may have different `Message` types, as long as they're convertible to the root type via `From`.
//!
//! The second way of registering stories is as follows:
//!
//! You've a `my_view` module which declares a button view
//! ```ignore
//! mod my_view {
//!     use iced::*;
//!
//!     // This view has the state of the button
//!     pub struct MyView {
//!         button_state: iced::button::State,
//!     }
//!
//!     // This view fires a `Message::ButtonClicked` message
//!     #[derive(Clone, Debug)]
//!     pub enum Message {
//!         ButtonClicked,
//!     }
//!
//!     impl MyView {
//!         pub fn new() -> Self {
//!             Self {
//!                 button_state: iced::button::State::default(),
//!             }
//!         }
//!
//!         pub fn view(&mut self) -> Element<Message> {
//!             Button::new(&mut self.button_state, Text::new("Hello world"))
//!                 .on_press(Message::ButtonClicked)
//!                 .into()
//!         }
//!     }
//!
//!     // You will declare a `story` module, which may be conditionally compiled on your set-up
//!     pub mod story {
//!         use audio_processor_iced_storybook::StoryView;
//!
//!         use super::*;
//!
//!         // You will declare some helper types
//!         struct Story(MyView);
//!         pub fn default() -> Story {
//!             Story::default()
//!         }
//!
//!         // You will implement the `StoryView` trait for your story. This will be parameterized over the `Message` type,
//!         // however it doesn't have to be a global type, as long as the root type is convertible to/from this.
//!         impl StoryView<Message> for Story {
//!             // You may implement an update function for your story
//!             fn update(&mut self, _message: Message) -> Command<Message> { Command::none() }
//!
//!             // You will implement the view function
//!             fn view(&mut self) -> Element<Message> {
//!                 self.0.view()
//!             }
//!         }
//!     }
//! }
//!
//! // In order to have different message types, you'll implement a "super-type" for Message, which derives `From` and
//! // `TryInto`
//!
//! use derive_more::{From, TryInto}; // <- You need this to derive `From`/`TryInto` automatically for the child message
//!
//! #[derive(Debug, From, Clone, TryInto)]
//! enum Message {
//!     MyView(my_view::Message),
//!     None(()) // <- Adding a `None(())` will let you continue using stateless stories as well.
//! }
//!
//! // examples/stories.rs
//! fn main() {
//!     audio_processor_iced_storybook::builder::<Message>()
//!         // You will register the story with `story` rather than `story_fn`.
//!         .story("MyView - default", my_view::story::default())
//!         .run()
//!         .unwrap();
//! }
//! ```
//!
//! For better examples, see [`crates/plugin-host-gui2`](https://github.com/yamadapc/augmented-audio/tree/master/crates/plugin-host-gui2).
//!

use std::fmt::Debug;

use iced::{
    widget::{Button, Column, Container, Row, Text},
    Application, Command, Element, Length, Subscription,
};

use audio_processor_iced_design_system::{menu_list, spacing::Spacing, style};

pub use crate::model::Options;
pub use crate::model::Story;
use crate::sidebar::SelectedStory;
pub use crate::{model::ConvertingStoryView, model::StoryView};

mod model;
mod sidebar;

pub struct OptionsBuilder<StoryMessage> {
    stories: Vec<Story<StoryMessage>>,
}

impl<StoryMessage: 'static + Debug + Clone + Send> OptionsBuilder<StoryMessage> {
    pub fn story_fn<StoryFn>(mut self, title: impl Into<String>, content: StoryFn) -> Self
    where
        StoryFn: 'static + Fn() -> Element<'static, StoryMessage>,
    {
        self.stories.push(Story {
            id: self.stories.len().to_string(),
            title: title.into(),
            renderer: Box::new(content),
        });
        self
    }

    pub fn story<View, ViewMessage, Title>(mut self, title: Title, content: View) -> Self
    where
        View: 'static + StoryView<ViewMessage>,
        ViewMessage: 'static,
        Title: Into<String>,
        ConvertingStoryView<ViewMessage>: StoryView<StoryMessage>,
    {
        self.stories.push(Story {
            id: self.stories.len().to_string(),
            title: title.into(),
            renderer: Box::new(ConvertingStoryView::new(Box::new(content))),
        });
        self
    }

    pub fn run(self) -> iced::Result {
        let options = Options {
            stories: self.stories,
        };
        main(options)
    }
}

pub fn builder<StoryMessage: 'static + Debug + Clone + Send>() -> OptionsBuilder<StoryMessage> {
    OptionsBuilder { stories: vec![] }
}

pub fn main<StoryMessage: 'static + Debug + Clone + Send>(
    options: Options<StoryMessage>,
) -> iced::Result {
    wisual_logger::init_from_env();
    log::info!("Initializing iced-storybook");
    StorybookApp::run(iced::Settings {
        antialiasing: true,
        default_text_size: Spacing::default_font_size() as f32,
        ..iced::Settings::with_flags(options)
    })
}

#[derive(Debug, Clone)]
pub enum Message<StoryMessage> {
    Sidebar(sidebar::Message),
    Child(StoryMessage),
    ToggleLogging,
}

struct StorybookApp<StoryMessage> {
    options: Options<StoryMessage>,
    selected_story: Option<SelectedStory>,
    sidebar: sidebar::SidebarView,
    last_messages: Vec<StoryMessage>,
    logging_enabled: bool,
}

impl<StoryMessage: 'static + Debug + Clone + Send> Application for StorybookApp<StoryMessage> {
    type Executor = iced::executor::Default;
    type Message = Message<StoryMessage>;
    type Flags = Options<StoryMessage>;
    type Theme = iced::theme::Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let options = flags;
        let mut app = StorybookApp {
            sidebar: sidebar::SidebarView::new(&options),
            selected_story: None,
            options,
            last_messages: vec![],
            logging_enabled: false,
        };
        let command = app.select_first_story();

        (app, command)
    }

    fn title(&self) -> String {
        String::from("Storybook")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Sidebar(message) => {
                match &message {
                    sidebar::Message::MenuList(menu_list::Message::Selected { option, .. }) => {
                        self.selected_story = Some(option.clone());
                    }
                }

                self.sidebar.update(message).map(Message::Sidebar)
            }
            Message::Child(child_message) => {
                if self.logging_enabled {
                    self.last_messages.push(child_message.clone());
                    if self.last_messages.len() > 7 {
                        self.last_messages = self
                            .last_messages
                            .iter()
                            .rev()
                            .take(7)
                            .rev()
                            .cloned()
                            .collect();
                    }
                }

                if let Some(story) = find_story_mut(&self.selected_story, &mut self.options) {
                    story.renderer.update(child_message).map(Message::Child)
                } else {
                    Command::none()
                }
            }
            Message::ToggleLogging => {
                self.logging_enabled = !self.logging_enabled;
                Command::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let mut subscriptions = vec![];
        if let Some(story) = find_story(&self.selected_story, &self.options) {
            subscriptions.push(story.renderer.subscription().map(Message::Child));
        }
        Subscription::batch(subscriptions)
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let story = find_story(&self.selected_story, &self.options)
            .map(|story| story.renderer.view().map(Message::Child));
        let story_view = Container::new(Row::with_children(vec![story.unwrap_or_else(|| {
            Container::new(Text::new("Select a story"))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_y()
                .center_x()
                .into()
        })]))
        .width(Length::Fill)
        .height(Length::Fill)
        .into();

        let content = Row::with_children(vec![
            Container::new(self.sidebar.view().map(Message::Sidebar))
                .width(Length::Fixed(200.0))
                .height(Length::Fill)
                .into(),
            story_view,
        ])
        .height(Length::Fill);

        let bottom_panel = Container::new(Column::with_children(vec![
            Row::with_children(vec![
                Text::new(" ======== Messages log ========").into(),
                Button::new(Text::new(if self.logging_enabled {
                    "Disable log"
                } else {
                    "Enable log"
                }))
                .on_press(Message::ToggleLogging)
                .style(style::Button::default().into())
                .into(),
            ])
            .into(),
            Column::with_children(
                self.last_messages
                    .iter()
                    .map(|message| Text::new(format!("{:?}", message)).into())
                    .collect(),
            )
            .into(),
        ]))
        .style(style::Container1::default().border())
        .padding(Spacing::base_spacing())
        .height(Length::Fixed(200.0))
        .width(Length::Fill);

        Container::new(Column::with_children(vec![
            content.into(),
            bottom_panel.into(),
        ]))
        .width(Length::Fill)
        .height(Length::Fill)
        .style(style::Container0::default())
        .into()
    }
}

impl<StoryMessage: 'static + Debug + Clone + Send> StorybookApp<StoryMessage> {
    fn select_first_story(&mut self) -> Command<Message<StoryMessage>> {
        if !self.options.stories.is_empty() {
            let message =
                Message::Sidebar(sidebar::Message::MenuList(menu_list::Message::Selected {
                    index: 0,
                    option: SelectedStory {
                        id: self.options.stories[0].id.to_string(),
                    },
                }));
            self.update(message)
        } else {
            Command::none()
        }
    }
}

fn find_story<'a, StoryMessage>(
    selected_story: &'a Option<SelectedStory>,
    options: &'a Options<StoryMessage>,
) -> Option<&'a Story<StoryMessage>> {
    match &selected_story {
        Some(selected_story) => options
            .stories
            .iter()
            .find(|story| story.id == selected_story.id),
        None => None,
    }
}

fn find_story_mut<'a, StoryMessage>(
    selected_story: &'a Option<SelectedStory>,
    options: &'a mut Options<StoryMessage>,
) -> Option<&'a mut Story<StoryMessage>> {
    match &selected_story {
        Some(selected_story) => options
            .stories
            .iter_mut()
            .find(|story| story.id == selected_story.id),
        None => None,
    }
}
