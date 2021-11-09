use std::fmt::Debug;

use iced::{
    button, Application, Button, Column, Command, Container, Element, Length, Row, Subscription,
    Text,
};

use audio_processor_iced_design_system::menu_list;
use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style;
pub use model::Options;
pub use model::Story;

pub use crate::model::ConvertingStoryView;
pub use crate::model::StoryView;
use crate::sidebar::SelectedStory;

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
        default_text_size: audio_processor_iced_design_system::spacing::Spacing::default_font_size(
        ),
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
    selected_story: Option<sidebar::SelectedStory>,
    sidebar: sidebar::SidebarView,
    last_messages: Vec<StoryMessage>,
    logging_enabled: bool,
    log_button_state: button::State,
}

impl<StoryMessage: 'static + Debug + Clone + Send> Application for StorybookApp<StoryMessage> {
    type Executor = iced::executor::Default;
    type Message = Message<StoryMessage>;
    type Flags = Options<StoryMessage>;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let options = flags;
        (
            StorybookApp {
                sidebar: sidebar::SidebarView::new(&options),
                selected_story: None,
                options,
                last_messages: vec![],
                logging_enabled: false,
                log_button_state: Default::default(),
            },
            Command::none(),
        )
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

    fn view(&mut self) -> Element<'_, Self::Message> {
        let story = find_story_mut(&self.selected_story, &mut self.options)
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
                .width(Length::Units(200))
                .height(Length::Fill)
                .into(),
            story_view,
        ])
        .height(Length::Fill);

        let bottom_panel = Container::new(Column::with_children(vec![
            Row::with_children(vec![
                Text::new(" ======== Messages log ========").into(),
                Button::new(
                    &mut self.log_button_state,
                    Text::new(if self.logging_enabled {
                        "Disable log"
                    } else {
                        "Enable log"
                    }),
                )
                .on_press(Message::ToggleLogging)
                .style(style::Button)
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
        .height(Length::Units(200))
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
