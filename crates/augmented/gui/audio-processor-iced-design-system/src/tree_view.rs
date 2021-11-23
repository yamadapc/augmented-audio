use iced::{Column, Element, Length};

pub use item::ItemState;

use crate::updatable::Updatable;

pub struct State<InnerState: Updatable> {
    items: Vec<item::ItemState<InnerState>>,
}

impl<InnerState: Updatable> State<InnerState> {
    pub fn new(items: Vec<item::ItemState<InnerState>>) -> Self {
        State { items }
    }
}

#[derive(Debug, Clone)]
pub enum Message<InnerMessage> {
    Child(usize, item::Message<InnerMessage>),
}

impl<InnerState: Updatable + 'static> State<InnerState> {
    pub fn update(&mut self, msg: Message<InnerState::Message>) {
        match msg {
            Message::Child(index, msg) => {
                self.items[index].update(msg);
            }
        }
    }

    pub fn view(&mut self) -> Element<Message<InnerState::Message>> {
        let children = self
            .items
            .iter_mut()
            .enumerate()
            .map(|(index, item)| item.view().map(move |msg| Message::Child(index, msg)))
            .collect();
        Column::with_children(children).width(Length::Fill).into()
    }
}

mod item {
    use std::fmt::Debug;

    use iced::{Button, Column, Container, Element, Text};

    use crate::spacing::Spacing;
    use crate::updatable::Updatable;

    #[derive(Debug, Clone)]
    pub enum Message<InnerMessage> {
        Toggle,
        Child(usize, Box<Message<InnerMessage>>),
        Inner(InnerMessage),
    }

    #[derive(Debug, Clone)]
    pub enum ItemState<InnerState> {
        Item {
            title: String,
            state: InnerState,
        },
        Parent {
            title: String,
            children: Vec<ItemState<InnerState>>,
            button_state: iced::button::State,
            is_collapsed: bool,
        },
    }

    impl ItemState<()> {
        pub fn child(title: String) -> Self {
            ItemState::Item { title, state: () }
        }
    }

    impl<InnerState> ItemState<InnerState>
    where
        InnerState: Updatable + 'static,
    {
        pub fn child_with(title: String, state: InnerState) -> Self {
            ItemState::Item { title, state }
        }

        pub fn parent(title: String, children: Vec<ItemState<InnerState>>) -> Self {
            ItemState::Parent {
                title,
                children,
                button_state: iced::button::State::default(),
                is_collapsed: false,
            }
        }

        pub fn update(&mut self, message: Message<InnerState::Message>) {
            match self {
                ItemState::Parent {
                    is_collapsed,
                    children,
                    ..
                } => match message {
                    Message::Toggle => {
                        *is_collapsed = !*is_collapsed;
                    }
                    Message::Child(index, msg) => {
                        children[index].update(*msg);
                    }
                    _ => {}
                },
                ItemState::Item { state, .. } => {
                    if let Message::Inner(inner) = message {
                        state.update(inner);
                    }
                }
            }
        }

        pub fn view(&mut self) -> Element<Message<InnerState::Message>> {
            match self {
                ItemState::Item { title, .. } => Text::new(&*title).into(),
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
                                    .map(move |msg| Message::Child(index, Box::new(msg)))
                            })
                            .collect(),
                    ))
                    .padding([0, 0, 0, Spacing::base_spacing()])
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
