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
}

enum PaneState {
    Sidebar,
    Content,
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
                        a: Box::new(pane_grid::Configuration::Pane(PaneState::Content)),
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
        }
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let panel = PaneGrid::new(&mut self.pane_state, |_pane, state| match state {
            PaneState::Sidebar => Sidebar::view().into(),
            PaneState::Content => Content::view().into(),
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

struct Content;

impl Content {
    fn view() -> Element<'static, Message> {
        let items = vec![
            TreeViewItem::Item(Text::new("Heading 1")),
            TreeViewItem::Parent {
                title: String::from("Heading 2"),
                children: vec![
                    TreeViewItem::Item(Text::new("Child 1")),
                    TreeViewItem::Parent {
                        title: String::from("Child 2"),
                        children: vec![TreeViewItem::Item(Text::new("Sub-child 1"))],
                    },
                    TreeViewItem::Item(Text::new("Child 3")),
                ],
            },
            TreeViewItem::Item(Text::new("Heading 3")),
        ];
        let mut tree_view = TreeView::new(items);

        Container::new(tree_view.view())
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(audio_processor_iced_design_system::container::style::Container)
            .into()
    }
}

struct TreeView<Item> {
    items: Vec<TreeViewItem<Item>>,
}

impl<Item> TreeView<Item> {
    pub fn new(items: Vec<TreeViewItem<Item>>) -> Self {
        TreeView { items }
    }
}

enum TreeViewItem<Item> {
    Item(Item),
    Parent {
        title: String,
        children: Vec<TreeViewItem<Item>>,
    },
}

impl<Item> TreeView<Item>
where
    Item: Into<Element<'static, Message>>,
{
    fn view(self) -> Element<'static, Message> {
        Column::with_children(
            self.items
                .into_iter()
                .map(|item| TreeView::render_item(item))
                .collect(),
        )
        .width(Length::Fill)
        .into()
    }

    fn render_item(tree_item: TreeViewItem<Item>) -> Element<'static, Message> {
        match tree_item {
            TreeViewItem::Item(inner) => inner.into(),
            TreeViewItem::Parent { title, children } => {
                let child_elements = TreeView::new(children);
                let child_elements = Container::new(child_elements.view()).into();
                Column::with_children(vec![Text::new(title).into(), child_elements]).into()
            }
        }
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
