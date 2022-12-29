use std::convert::TryInto;

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
use iced::{Command, Element, Subscription};

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

    fn view(&self) -> Element<StoryMessage>;
}

impl<StoryMessage, F> StoryView<StoryMessage> for F
where
    F: 'static + Fn() -> Element<'static, StoryMessage>,
{
    fn view(&self) -> Element<StoryMessage> {
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

    fn view(&self) -> Element<TargetMessage> {
        self.inner.view().map(TargetMessage::from)
    }
}
