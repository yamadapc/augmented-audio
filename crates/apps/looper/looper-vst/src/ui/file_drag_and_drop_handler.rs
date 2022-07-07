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
use iced::Subscription;

use super::Message;

pub fn drag_and_drop_subscription() -> Subscription<Message> {
    iced_native::subscription::events().map(iced_event_to_drag_message)
}

fn iced_event_to_drag_message(event: iced_native::Event) -> Message {
    if let iced_native::Event::Window(event) = event {
        match event {
            iced_native::window::Event::FileHovered(path) => {
                log::info!("Received file hovered {:?}", path);
                Message::FileHover(path)
            }
            iced_native::window::Event::FileDropped(path) => {
                log::info!("Received file drop {:?}", path);
                Message::FileDropped(path)
            }
            _ => Message::None,
        }
    } else {
        Message::None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iced_event_to_drag_message_returns_none_when_not_relevant_window_event() {
        let iced_event = iced_native::Event::Window(iced_native::window::Event::Focused);
        let result = iced_event_to_drag_message(iced_event);
        assert_eq!(result, Message::None);
    }

    #[test]
    fn test_iced_event_to_drag_message_returns_none_when_not_relevant_event() {
        let iced_event =
            iced_native::Event::Keyboard(iced_native::keyboard::Event::CharacterReceived('a'));
        let result = iced_event_to_drag_message(iced_event);
        assert_eq!(result, Message::None);
    }

    #[test]
    fn test_iced_event_to_drag_message_returns_hover_events() {
        let iced_event =
            iced_native::Event::Window(iced_native::window::Event::FileHovered("something".into()));
        let result = iced_event_to_drag_message(iced_event);
        assert_eq!(result, Message::FileHover("something".into()));
    }

    #[test]
    fn test_iced_event_to_drag_message_returns_dropped_events() {
        let iced_event =
            iced_native::Event::Window(iced_native::window::Event::FileDropped("something".into()));
        let result = iced_event_to_drag_message(iced_event);
        assert_eq!(result, Message::FileDropped("something".into()));
    }
}
