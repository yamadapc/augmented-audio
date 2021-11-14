use iced::{Column, Element, Row, Text};

#[derive(Clone, Debug)]
pub enum Message {}

pub struct TabsState {}

pub struct Tabs {}

impl Tabs {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> Element<Message> {
        Column::with_children(vec![
            Row::with_children(vec![
                Text::new("Tabs").into(),
                Text::new("Tabs").into(),
                Text::new("Tabs").into(),
            ])
            .into(),
            Text::new("Content").into(),
        ])
        .into()
    }
}

#[cfg(feature = "story")]
pub mod story {
    use iced::Command;

    use audio_processor_iced_storybook::StoryView;

    use super::*;

    struct TabsStory {
        tabs: Tabs,
    }

    pub fn default() -> impl StoryView<Message> {
        TabsStory { tabs: Tabs::new() }
    }

    impl StoryView<Message> for TabsStory {
        fn update(&mut self, _message: Message) -> Command<Message> {
            Command::none()
        }

        fn view(&mut self) -> Element<Message> {
            self.tabs.view()
        }
    }
}
