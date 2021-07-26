use iced::{Command, Element, Subscription};
use std::convert::TryInto;

pub struct Options<StoryMessage> {
    pub stories: Vec<Story<StoryMessage>>,
}

pub struct Story<StoryMessage> {
    pub id: String,
    pub title: String,
    pub renderer: Box<dyn StoryView<StoryMessage>>,
}

pub trait StoryView<StoryMessage> {
    fn update(&mut self, _message: StoryMessage) -> Command<StoryMessage> {
        Command::none()
    }

    fn subscription(&self) -> Subscription<StoryMessage> {
        Subscription::none()
    }

    fn view(&mut self) -> Element<StoryMessage>;
}

impl<StoryMessage, F> StoryView<StoryMessage> for F
where
    F: 'static + Fn() -> Element<'static, StoryMessage>,
{
    fn view(&mut self) -> Element<StoryMessage> {
        self()
    }
}

pub struct ConvertingStoryView<InnerMessage> {
    inner: Box<dyn StoryView<InnerMessage>>,
}

impl<InnerMessage> ConvertingStoryView<InnerMessage> {
    pub fn new(inner: Box<dyn StoryView<InnerMessage>>) -> Self {
        ConvertingStoryView { inner }
    }
}

impl<InnerMessage, TargetMessage> StoryView<TargetMessage> for ConvertingStoryView<InnerMessage>
where
    InnerMessage: 'static,
    TargetMessage: 'static,
    TargetMessage: From<InnerMessage>,
    TargetMessage: TryInto<InnerMessage>,
{
    fn update(&mut self, message: TargetMessage) -> Command<TargetMessage> {
        if let Ok(inner_message) = message.try_into() {
            self.inner.update(inner_message).map(|inner| inner.into())
        } else {
            Command::none()
        }
    }

    fn subscription(&self) -> Subscription<TargetMessage> {
        self.inner.subscription().map(|inner| inner.into())
    }

    fn view(&mut self) -> Element<TargetMessage> {
        self.inner.view().map(TargetMessage::from)
    }
}
