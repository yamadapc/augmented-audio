use crate::Options;
use audio_processor_iced_design_system::menu_list;
use audio_processor_iced_design_system::style;
use iced::{Command, Element, Row, Rule, Text};

#[derive(Debug, Clone)]
pub enum Message {
    MenuList(menu_list::Message<SelectedStory>),
}

#[derive(Debug, Clone)]
pub struct SelectedStory {
    pub id: String,
}

pub struct SidebarView {
    menu_list: menu_list::State<String, SelectedStory>,
}

impl SidebarView {
    pub fn new<Inner>(options: &Options<Inner>) -> Self {
        let items = options
            .stories
            .iter()
            .map(|story| {
                (
                    story.title.clone(),
                    SelectedStory {
                        id: story.id.clone(),
                    },
                )
            })
            .collect();
        Self {
            menu_list: menu_list::State::new(items, None),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::MenuList(message) => {
                self.menu_list.update(message);
            }
        }
        Command::none()
    }

    pub fn view(&mut self) -> Element<Message> {
        let container = self
            .menu_list
            .view(|text| Text::new(&*text).into())
            .map(|msg| Message::MenuList(msg))
            .into();

        let rule = Rule::vertical(1).style(style::Rule).into();
        return Row::with_children(vec![container, rule]).into();
    }
}
