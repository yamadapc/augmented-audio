use iced::{Column, Element, Length};

pub use item::ItemState;

pub struct State {
    items: Vec<item::ItemState>,
}

impl State {
    pub fn new(items: Vec<item::ItemState>) -> Self {
        State { items }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ChildMessage(usize, item::Message),
}

impl State {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::ChildMessage(index, msg) => {
                self.items[index].update(msg);
            }
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        let children = self
            .items
            .iter_mut()
            .enumerate()
            .map(|(index, item)| {
                item.view()
                    .map(move |msg| Message::ChildMessage(index, msg))
            })
            .collect();
        Column::with_children(children).width(Length::Fill).into()
    }
}

mod item {
    use iced::{Button, Column, Container, Element, Text};

    #[derive(Debug, Clone)]
    pub enum Message {
        Toggle,
        ChildToggled(usize, Box<Message>),
    }

    #[derive(Debug, Clone)]
    pub enum ItemState {
        Item(String),
        Parent {
            title: String,
            children: Vec<ItemState>,
            button_state: iced::button::State,
            is_collapsed: bool,
        },
    }

    impl ItemState {
        pub fn parent(title: String, children: Vec<ItemState>) -> Self {
            ItemState::Parent {
                title,
                children,
                button_state: iced::button::State::default(),
                is_collapsed: false,
            }
        }

        pub fn update(&mut self, message: Message) {
            match self {
                ItemState::Item(_) => {}
                ItemState::Parent {
                    is_collapsed,
                    children,
                    ..
                } => match message {
                    Message::Toggle => {
                        *is_collapsed = !*is_collapsed;
                    }
                    Message::ChildToggled(index, msg) => {
                        children[index].update(*msg);
                    }
                },
            }
        }

        pub fn view(&mut self) -> Element<Message> {
            match self {
                ItemState::Item(inner) => Text::new(&*inner).into(),
                ItemState::Parent {
                    title,
                    children,
                    button_state,
                    is_collapsed,
                } => {
                    let child_elements = Container::new(Column::with_children(
                        children
                            .iter_mut()
                            .enumerate()
                            .map(|(index, item)| {
                                item.view()
                                    .map(move |msg| Message::ChildToggled(index, Box::new(msg)))
                            })
                            .collect(),
                    ))
                    .padding([0, 0, 0, 30])
                    .into();

                    let toggle_button = Button::new(button_state, Text::new(&*title))
                        .padding(0)
                        .style(crate::style::ChromelessButton)
                        .on_press(Message::Toggle)
                        .into();

                    let children = if *is_collapsed {
                        vec![toggle_button]
                    } else {
                        vec![toggle_button, child_elements]
                    };

                    Column::with_children(children).into()
                }
            }
        }
    }
}
