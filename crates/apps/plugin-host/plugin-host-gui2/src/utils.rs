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
use iced::Command;

use crate::ui::main_content_view::status_bar::StatusBar;
use crate::ui::main_content_view::Message;

#[macro_export]
macro_rules! string_vec {
    ($($x:expr),*) => (vec![$($x.to_string()), *])
}

pub fn get_version() -> String {
    format!(
        "{}-{}-{}",
        env!("PROFILE"),
        env!("CARGO_PKG_VERSION"),
        env!("GIT_REV_SHORT")
    )
}

pub fn command_message<Message: 'static + Send>(message: Message) -> iced::Command<Message> {
    iced::Command::perform(async move { message }, |message| message)
}

pub fn set_status_bar(status: StatusBar) -> Command<Message> {
    Command::perform(iced_futures::futures::future::ready(()), move |_| {
        Message::SetStatus(status)
    })
}
