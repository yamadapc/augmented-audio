use iced::Subscription;

use super::Message;

pub fn drag_and_drop_subscription() -> Subscription<Message> {
    iced_native::subscription::events().map(|event| {
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
    })
}
