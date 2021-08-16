use iced::{Button, Command, Container, Element, Length, Row};

use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::{colors, style};
use pause::Pause;
use stop::Stop;
use triangle::Triangle;

pub mod pause;
pub mod stop;
pub mod triangle;

pub struct TransportControlsView {
    pause: Pause,
    pause_button_state: iced::button::State,
    triangle: Triangle,
    play_button_state: iced::button::State,
    stop: Stop,
    stop_button_state: iced::button::State,
}

#[derive(Clone, Debug)]
pub enum Message {
    Play,
    Pause,
    Stop,
    None,
}

impl Default for TransportControlsView {
    fn default() -> Self {
        Self::new()
    }
}

impl TransportControlsView {
    pub fn new() -> Self {
        TransportControlsView {
            pause: Pause::new(),
            pause_button_state: iced::button::State::new(),
            triangle: Triangle::new(),
            play_button_state: iced::button::State::new(),
            stop: Stop::new(),
            stop_button_state: iced::button::State::new(),
        }
    }

    pub fn update(&self, _message: Message) -> Command<Message> {
        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
        let pause = Button::new(
            &mut self.pause_button_state,
            self.pause
                .color(colors::Colors::text())
                .hover(colors::yellow())
                .view()
                .map(|_| Message::None),
        )
        .on_press(Message::Pause)
        .style(style::ChromelessButton)
        .padding(0)
        .width(Length::Units(Spacing::small_control_size()))
        .height(Length::Units(Spacing::small_control_size()))
        .into();
        let triangle = Button::new(
            &mut self.play_button_state,
            self.triangle
                .color(colors::Colors::text())
                .hover(colors::green())
                .view()
                .map(|_| Message::None),
        )
        .style(style::ChromelessButton)
        .on_press(Message::Play)
        .padding(0)
        .width(Length::Units(Spacing::small_control_size()))
        .height(Length::Units(Spacing::small_control_size()))
        .into();
        let square = Button::new(
            &mut self.stop_button_state,
            self.stop
                .color(colors::Colors::text())
                .hover(colors::red())
                .view()
                .map(|_| Message::None),
        )
        .on_press(Message::Stop)
        .style(style::ChromelessButton)
        .padding(0)
        .width(Length::Units(Spacing::small_control_size()))
        .height(Length::Units(Spacing::small_control_size()))
        .into();

        Container::new(
            Row::with_children(vec![pause, triangle, square]).spacing(Spacing::base_spacing()),
        )
        .center_x()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

#[cfg(feature = "story")]
pub mod story {
    use audio_processor_iced_storybook::StoryView;

    use super::*;

    pub fn default() -> Story {
        Story {
            transport: TransportControlsView::new(),
        }
    }

    pub struct Story {
        transport: TransportControlsView,
    }

    impl StoryView<Message> for Story {
        fn update(&mut self, message: Message) -> Command<Message> {
            self.transport.update(message)
        }

        fn view(&mut self) -> Element<Message> {
            self.transport.view()
        }
    }
}
