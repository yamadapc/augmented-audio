use iced::pane_grid::Axis;
use iced::{
    widget, Application, Clipboard, Column, Command, Container, Element, Length, PaneGrid, Row,
    Rule, Settings, Text,
};
use widget::pane_grid;

use audio_processor_iced_design_system::container::style as container_style;
use audio_processor_iced_design_system::router::RouterState;
use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style as audio_style;
use audio_processor_iced_design_system::{menu_list, tree_view};
use iced::futures::FutureExt;
use std::cell::RefCell;
use std::collections::HashMap;

fn main() -> iced::Result {
    wisual_logger::init_from_env();
    log::info!("Initializing app");
    WalkthroughApp::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    PaneResized(pane_grid::ResizeEvent),
    Content(tree_view::Message<()>),
    Sidebar(menu_list::Message),
}

enum PaneState {
    Sidebar(Sidebar),
    Content(Content),
    Bottom,
}

struct WalkthroughApp {
    router_state: RouterState<()>,
    pane_state: pane_grid::State<PaneState>,
}

impl Application for WalkthroughApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut router_state = RouterState::new(String::from("/0"), HashMap::new());
        router_state.add_route(String::from("/0"), ());
        router_state.add_route(String::from("/1"), ());
        router_state.add_route(String::from("/2"), ());
        router_state.add_route(String::from("/3"), ());

        (
            WalkthroughApp {
                router_state,
                pane_state: pane_grid::State::with_configuration(pane_grid::Configuration::Split {
                    axis: Axis::Horizontal,
                    ratio: 0.8,
                    a: Box::new(pane_grid::Configuration::Split {
                        axis: Axis::Vertical,
                        ratio: 0.2,
                        a: Box::new(pane_grid::Configuration::Pane(PaneState::Sidebar(
                            Sidebar::new(),
                        ))),
                        b: Box::new(pane_grid::Configuration::Pane(PaneState::Content(
                            Content::new(),
                        ))),
                    }),
                    b: Box::new(pane_grid::Configuration::Pane(PaneState::Bottom)),
                }),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Walkthrough")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        match message {
            Message::PaneResized(resized) => self.pane_state.resize(&resized.split, resized.ratio),
            Message::Content(msg) => {
                for (_, state) in self.pane_state.iter_mut() {
                    if let PaneState::Content(content) = state {
                        content.update(msg.clone())
                    }
                }
            }
            Message::Sidebar(msg) => {
                for (_, state) in self.pane_state.iter_mut() {
                    if let PaneState::Sidebar(sidebar) = state {
                        sidebar.update(msg.clone())
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let panel = PaneGrid::new(&mut self.pane_state, |_pane, state| match state {
            PaneState::Sidebar(sidebar) => sidebar.view().into(),
            PaneState::Content(content) => {
                let router_state = &mut self.router_state;
                Column::with_children(vec![
                    router_state.route("/0", |_| content.view()),
                    router_state.route("/0", |_| content.view()),
                    router_state.route("/0", |_| content.view()),
                    router_state.route("/0", |_| content.view()),
                ])
                .into()
            }
            PaneState::Bottom => BottomPanel::view().into(),
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .style(audio_style::PaneGrid)
        .on_resize(5, |resize_event| Message::PaneResized(resize_event));

        Container::new(panel)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(container_style::Container0)
            .into()
    }
}

struct Content {
    tree_view: tree_view::State<()>,
}

impl Content {
    fn new() -> Self {
        let items = vec![
            tree_view::ItemState::child(String::from("Heading 1")),
            tree_view::ItemState::parent(
                String::from("Heading 2"),
                vec![
                    tree_view::ItemState::child(String::from("Child 1")),
                    tree_view::ItemState::parent(
                        String::from("Child 2"),
                        vec![tree_view::ItemState::child(String::from("Sub-child 1"))],
                    ),
                    tree_view::ItemState::child(String::from("Child 3")),
                ],
            ),
            tree_view::ItemState::child(String::from("Heading 3")),
        ];
        let tree_view = tree_view::State::new(items);

        Content { tree_view }
    }

    fn update(&mut self, message: tree_view::Message<()>) {
        self.tree_view.update(message);
    }

    fn view(&mut self) -> Element<Message> {
        let tree_view = self.tree_view.view().map(|msg| Message::Content(msg));

        Container::new(tree_view)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .padding(Spacing::base_spacing())
            .style(container_style::Container1)
            .into()
    }
}

struct Sidebar {
    menu_list: menu_list::State<String>,
}

impl Sidebar {
    pub fn new() -> Self {
        Sidebar {
            menu_list: menu_list::State::new(
                vec![
                    String::from("Menu item 1"),
                    String::from("Menu item 2"),
                    String::from("Menu item 3"),
                    String::from("Menu item 4"),
                    String::from("Menu item 5"),
                ],
                Some(0),
            ),
        }
    }

    fn update(&mut self, message: menu_list::Message) {
        self.menu_list.update(message.clone());
        match message {
            menu_list::Message::Selected(_) => {}
        }
    }

    fn view(&mut self) -> Element<Message> {
        let container = self
            .menu_list
            .view(|text| Text::new(&*text).into())
            .map(|msg| Message::Sidebar(msg))
            .into();

        let rule = Rule::vertical(1).style(audio_style::Rule).into();
        return Row::with_children(vec![container, rule]).into();
    }
}

struct BottomPanel;

impl BottomPanel {
    fn view() -> Element<'static, Message> {
        Column::with_children(vec![
            Rule::horizontal(1).style(audio_style::Rule).into(),
            Container::new(Text::new("Hello"))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .style(container_style::Container0)
                .into(),
        ])
        .into()
    }
}
