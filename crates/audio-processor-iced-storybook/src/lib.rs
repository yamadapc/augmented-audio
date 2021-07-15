use std::fmt::Debug;

use iced::{Application, Clipboard, Command, Container, Element, Length, Row, Text};

use audio_processor_iced_design_system::menu_list;
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
    StorybookApp::run(iced::Settings::with_flags(options))
}

#[derive(Debug, Clone)]
pub enum Message<StoryMessage> {
    Sidebar(sidebar::Message),
    Child(StoryMessage),
}

struct StorybookApp<StoryMessage> {
    options: Options<StoryMessage>,
    selected_story: Option<sidebar::SelectedStory>,
    sidebar: sidebar::SidebarView,
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
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Storybook")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::Sidebar(message) => {
                match &message {
                    sidebar::Message::MenuList(menu_list::Message::Selected { option, .. }) => {
                        self.selected_story = Some(option.clone());
                    }
                }

                self.sidebar
                    .update(message)
                    .map(|message| Message::Sidebar(message))
            }
            Message::Child(child_message) => {
                if let Some(story) = find_story(&self.selected_story, &mut self.options) {
                    story
                        .renderer
                        .update(child_message)
                        .map(|inner| Message::Child(inner))
                } else {
                    Command::none()
                }
            }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let story = find_story(&self.selected_story, &mut self.options)
            .map(|story| story.renderer.view().map(|msg| Message::Child(msg)));
        let story_view = Container::new(Row::with_children(vec![story.unwrap_or(
            Container::new(Text::new("Select a story"))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_y()
                .center_x()
                .into(),
        )]))
        .width(Length::Fill)
        .height(Length::Fill)
        .into();

        let content = Row::with_children(vec![
            Container::new(self.sidebar.view().map(|message| Message::Sidebar(message)))
                .width(Length::Units(200))
                .height(Length::Fill)
                .into(),
            story_view,
        ]);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(style::Container0)
            .into()
    }
}

fn find_story<'a, StoryMessage>(
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
