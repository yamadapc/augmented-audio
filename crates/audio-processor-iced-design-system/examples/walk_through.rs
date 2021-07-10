use audio_processor_iced_design_system::tree_view;
use iced::pane_grid::Axis;
use iced::{
    widget, Application, Clipboard, Column, Command, Container, Element, Length, PaneGrid, Row,
    Rule, Settings, Text,
};
use widget::pane_grid;

fn main() -> iced::Result {
    wisual_logger::init_from_env();
    log::info!("Initializing app");
    WalkthroughApp::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    PaneResized(pane_grid::ResizeEvent),
    Content(tree_view::Message),
}

enum PaneState {
    Sidebar,
    Content(Content),
    Bottom,
}

struct WalkthroughApp {
    pane_state: pane_grid::State<PaneState>,
}

impl Application for WalkthroughApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            WalkthroughApp {
                pane_state: pane_grid::State::with_configuration(pane_grid::Configuration::Split {
                    axis: Axis::Vertical,
                    ratio: 0.2,
                    a: Box::new(pane_grid::Configuration::Pane(PaneState::Sidebar)),
                    b: Box::new(pane_grid::Configuration::Split {
                        axis: Axis::Horizontal,
                        ratio: 0.8,
                        a: Box::new(pane_grid::Configuration::Pane(PaneState::Content(
                            Content::new(),
                        ))),
                        b: Box::new(pane_grid::Configuration::Pane(PaneState::Bottom)),
                    }),
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
        }
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let panel = PaneGrid::new(&mut self.pane_state, |_pane, state| match state {
            PaneState::Sidebar => Sidebar::view().into(),
            PaneState::Content(content) => content.view().into(),
            PaneState::Bottom => BottomPanel::view().into(),
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .style(style::PaneGrid)
        .on_resize(5, |resize_event| Message::PaneResized(resize_event));

        Container::new(panel)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(audio_processor_iced_design_system::container::style::Container)
            .into()
    }
}

mod style {
    use iced::pane_grid::Line;
    use iced::widget::pane_grid;
    use iced::widget::rule;
    use iced::Color;

    pub struct PaneGrid;

    impl pane_grid::StyleSheet for PaneGrid {
        fn picked_split(&self) -> Option<Line> {
            Option::Some(Line {
                color: Color::new(1.0, 1.0, 1.0, 0.8),
                width: 2.0,
            })
        }

        fn hovered_split(&self) -> Option<Line> {
            Option::Some(Line {
                color: Color::new(1.0, 1.0, 1.0, 0.8),
                width: 2.0,
            })
        }
    }

    pub(crate) struct Rule;

    impl rule::StyleSheet for Rule {
        fn style(&self) -> rule::Style {
            rule::Style {
                color: Color::new(1.0, 1.0, 1.0, 0.5),
                width: 1,
                radius: 0.0,
                fill_mode: rule::FillMode::Full,
            }
        }
    }
}

struct Content {
    tree_view: tree_view::State,
}

impl Content {
    fn new() -> Self {
        let items = vec![
            tree_view::ItemState::Item(String::from("Heading 1")),
            tree_view::ItemState::parent(
                String::from("Heading 2"),
                vec![
                    tree_view::ItemState::Item(String::from("Child 1")),
                    tree_view::ItemState::parent(
                        String::from("Child 2"),
                        vec![tree_view::ItemState::Item(String::from("Sub-child 1"))],
                    ),
                    tree_view::ItemState::Item(String::from("Child 3")),
                ],
            ),
            tree_view::ItemState::Item(String::from("Heading 3")),
        ];
        let tree_view = tree_view::State::new(items);

        Content { tree_view }
    }

    fn update(&mut self, message: tree_view::Message) {
        self.tree_view.update(message);
    }

    fn view(&mut self) -> Element<Message> {
        let tree_view = self.tree_view.view().map(|msg| Message::Content(msg));

        Container::new(tree_view)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .padding(30)
            .style(audio_processor_iced_design_system::container::style::Container)
            .into()
    }
}

struct Sidebar;

impl Sidebar {
    fn view() -> Element<'static, Message> {
        let container = Column::with_children(vec![
            Text::new("1").into(),
            Text::new("2").into(),
            Text::new("3").into(),
            Text::new("4").into(),
            Text::new("5").into(),
            Text::new("6").into(),
            Text::new("7").into(),
        ])
        .width(Length::Fill)
        .height(Length::Fill)
        .into();
        let rule = Rule::vertical(1).style(style::Rule).into();
        return Row::with_children(vec![container, rule]).into();
    }
}

struct BottomPanel;

impl BottomPanel {
    fn view() -> Element<'static, Message> {
        Column::with_children(vec![
            Rule::horizontal(1).style(style::Rule).into(),
            Container::new(Text::new("Hello"))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .style(audio_processor_iced_design_system::container::style::Container)
                .into(),
        ])
        .into()
    }
}
