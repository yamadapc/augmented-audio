use iced::Element;

use audio_processor_iced_design_system::style;

#[derive(Debug, Clone)]
pub enum Message {}

pub struct SequencerView {
    button_states: Vec<iced::button::State>,
}

impl Default for SequencerView {
    fn default() -> Self {
        SequencerView {
            button_states: (0..8).map(|_| iced::button::State::new()).collect(),
        }
    }
}

impl SequencerView {
    pub fn view(&mut self) -> Element<Message> {
        use iced::*;

        let buttons = self
            .button_states
            .iter_mut()
            .enumerate()
            .map(|(i, button_state)| {
                Button::new(button_state, Text::new(format!("{}", i + 1)))
                    .style(style::Button::default())
                    .width(Length::Fill)
                    .into()
            })
            .collect();

        Row::with_children(buttons).width(Length::Fill).into()
    }
}
