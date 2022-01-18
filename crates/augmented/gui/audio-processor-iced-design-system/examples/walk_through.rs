use iced::pane_grid::Axis;
use iced::{
    alignment, pick_list, widget, Alignment, Application, Button, Column, Command, Container,
    Element, Length, PaneGrid, Row, Rule, Settings, Text,
};
use widget::pane_grid;

use audio_processor_iced_design_system::container::HoverContainer;
use audio_processor_iced_design_system::knob::Knob;
use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style as audio_style;
use audio_processor_iced_design_system::tabs;
use audio_processor_iced_design_system::{knob as audio_knob, menu_list, tree_view};
use iced_audio::NormalParam;

fn main() -> iced::Result {
    wisual_logger::init_from_env();
    log::info!("Initializing app");
    WalkthroughApp::run(Settings {
        antialiasing: true,
        default_text_size: Spacing::default_font_size(),
        ..Settings::default()
    })
}

#[derive(Debug, Clone)]
enum Message {
    PaneResized(pane_grid::ResizeEvent),
    Content(ContentMessage),
    Sidebar(menu_list::Message<ContentView>),
}

enum PaneState {
    Sidebar(Sidebar),
    Content(Content),
    Bottom,
}

struct WalkthroughApp {
    #[allow(dead_code)]
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

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
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
                    let menu_list::Message::Selected { option, .. } = msg.clone();
                    if let PaneState::Content(content) = state {
                        content.update(ContentMessage::Selected(option))
                    }
                    if let PaneState::Sidebar(sidebar) = state {
                        sidebar.update(msg.clone())
                    }
                }
            }
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let panel = PaneGrid::new(&mut self.pane_state, |_pane, state| match state {
            PaneState::Sidebar(sidebar) => sidebar.view().into(),
            PaneState::Content(content) => Column::with_children(vec![content.view()]).into(),
            PaneState::Bottom => BottomPanel::view().into(),
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .style(audio_style::PaneGrid)
        .on_resize(5, |resize_event| Message::PaneResized(resize_event));

        Container::new(panel)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(audio_style::Container0::default())
            .into()
    }
}

#[derive(Clone, Debug)]
enum ContentMessage {
    PickList(String),
    TreeView(tree_view::Message<()>),
    Selected(ContentView),
    TabsView(tabs::Message<()>),
    None,
}

struct Content {
    content_view: ContentView,
    tree_view: tree_view::State<()>,
    pick_list_state_1: pick_list::State<String>,
    pick_list_state_2: pick_list::State<String>,
    pick_list_state_3: pick_list::State<String>,
    selected_option: usize,
    buttons_view: ButtonsView,
    tabs_view: tabs::State,
    knobs_view: KnobsView,
}

#[derive(Debug, Clone)]
enum ContentView {
    Dropdowns,
    SlidersAndKnobs,
    TreeView,
    Buttons,
    Tabs,
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
        let tabs_view = tabs::State::new();

        Content {
            content_view: ContentView::Tabs,
            tree_view,
            pick_list_state_1: pick_list::State::default(),
            pick_list_state_2: pick_list::State::default(),
            pick_list_state_3: pick_list::State::default(),
            selected_option: 0,
            buttons_view: ButtonsView::new(),
            tabs_view,
            knobs_view: KnobsView::new(),
        }
    }

    fn update(&mut self, message: ContentMessage) {
        match message {
            ContentMessage::TreeView(message) => self.tree_view.update(message),
            ContentMessage::PickList(selected) => {
                self.selected_option = if selected == "Option 1" { 0 } else { 1 };
            }
            ContentMessage::Selected(content_view) => {
                self.content_view = content_view;
            }
            ContentMessage::TabsView(message) => self.tabs_view.update(message),
            _ => {}
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = self.content_view();

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .padding(Spacing::base_spacing())
            .style(audio_style::Container1::default())
            .into()
    }

    fn content_view(&mut self) -> Element<Message> {
        match self.content_view {
            ContentView::Dropdowns => {
                let options = vec![String::from("Option 1"), String::from("Option 2")];
                let selected_option = options[self.selected_option].clone();
                Column::with_children(vec![
                    Container::new(dropdown_with_label(
                        &mut self.pick_list_state_1,
                        options.clone(),
                        selected_option.clone(),
                    ))
                    .padding([Spacing::base_spacing(), 0])
                    .into(),
                    Container::new(dropdown_with_label(
                        &mut self.pick_list_state_2,
                        options.clone(),
                        selected_option.clone(),
                    ))
                    .padding([Spacing::base_spacing(), 0])
                    .into(),
                    Container::new(dropdown_with_label(
                        &mut self.pick_list_state_3,
                        options,
                        selected_option,
                    ))
                    .padding([Spacing::base_spacing(), 0])
                    .into(),
                ])
                .into()
            }
            ContentView::SlidersAndKnobs => self
                .knobs_view
                .view()
                .map(|_| Message::Content(ContentMessage::None))
                .into(),
            ContentView::TreeView => self
                .tree_view
                .view()
                .map(|msg| Message::Content(ContentMessage::TreeView(msg))),
            ContentView::Buttons => self
                .buttons_view
                .view()
                .map(|_| Message::Content(ContentMessage::None)),
            ContentView::Tabs => self
                .tabs_view
                .view(vec![
                    tabs::Tab::new("Tab 1", Text::new("Tab 1 content")),
                    tabs::Tab::new("Tab 2", Text::new("Tab 2 content")),
                    tabs::Tab::new("Tab 3", Text::new("Tab 3 content")),
                ])
                .map(|msg| Message::Content(ContentMessage::TabsView(msg))),
        }
    }
}

struct KnobsView {
    knob_states: Vec<iced_audio::knob::State>,
    sliders: SlidersView,
}

impl KnobsView {
    pub fn new() -> Self {
        KnobsView {
            knob_states: vec![
                iced_audio::knob::State::new(Default::default()),
                iced_audio::knob::State::new(Default::default()),
                iced_audio::knob::State::new(Default::default()),
                iced_audio::knob::State::new(Default::default()),
                iced_audio::knob::State::new(Default::default()),
                iced_audio::knob::State::new(Default::default()),
            ],
            sliders: SlidersView::new(),
        }
    }
}

impl KnobsView {
    fn view(&mut self) -> Element<()> {
        let knobs = self
            .knob_states
            .iter_mut()
            .map(|knob_state| {
                HoverContainer::new(
                    Column::with_children(vec![
                        Text::new("Dry/Wet").size(Spacing::small_font_size()).into(),
                        Knob::new(knob_state, |_| {})
                            .size(Length::Units(Spacing::base_control_size()))
                            .style(audio_knob::style::Knob)
                            .into(),
                        Text::new("0%").size(Spacing::small_font_size()).into(),
                    ])
                    .align_items(Alignment::Center)
                    .spacing(Spacing::small_spacing()),
                )
                .style(audio_style::HoverContainer)
                .into()
            })
            .collect();
        Column::with_children(vec![
            Container::new(Text::new("Knobs"))
                .padding([0, 0, Spacing::base_spacing(), 0])
                .into(),
            Rule::horizontal(1).style(audio_style::Rule).into(),
            Container::new(Row::with_children(knobs).spacing(Spacing::base_spacing()))
                .width(Length::Fill)
                .center_x()
                .padding([Spacing::base_spacing(), 0])
                .into(),
            Rule::horizontal(1).style(audio_style::Rule).into(),
            Container::new(Text::new("Sliders"))
                .padding([Spacing::base_spacing(), 0])
                .into(),
            Rule::horizontal(1).style(audio_style::Rule).into(),
            self.sliders.view(),
        ])
        .into()
    }
}

struct SlidersView {
    state: Vec<iced_audio::v_slider::State>,
}

impl SlidersView {
    pub fn new() -> Self {
        SlidersView {
            state: (0..10)
                .map(|_| iced_audio::v_slider::State::new(NormalParam::default()))
                .collect(),
        }
    }

    pub fn view(&mut self) -> Element<()> {
        let elements = self
            .state
            .iter_mut()
            .map(|state| {
                iced_audio::VSlider::new(state, |_| ())
                    .style(audio_style::v_slider::VSlider)
                    .into()
            })
            .collect();
        Row::with_children(elements)
            .spacing(Spacing::base_spacing())
            .align_items(Alignment::Center)
            .width(Length::Fill)
            .padding([Spacing::base_spacing(), 0])
            .into()
    }
}

struct ButtonsView {
    default_button_state: iced::button::State,
    tiny_button_state: iced::button::State,
    chromeless_button_state: iced::button::State,
}

impl ButtonsView {
    pub fn new() -> Self {
        ButtonsView {
            default_button_state: iced::button::State::new(),
            tiny_button_state: iced::button::State::new(),
            chromeless_button_state: iced::button::State::new(),
        }
    }
}

impl ButtonsView {
    fn view(&mut self) -> Element<()> {
        Column::with_children(vec![
            Container::new(Text::new("Buttons"))
                .padding([0, 0, Spacing::base_spacing(), 0])
                .into(),
            Rule::horizontal(1).style(audio_style::Rule).into(),
            Container::new(Column::with_children(vec![
                Container::new(
                    Button::new(&mut self.default_button_state, Text::new("Default button"))
                        .on_press(())
                        .style(audio_style::Button::default()),
                )
                .padding([0, 0, Spacing::base_spacing(), 0])
                .into(),
                Container::new(
                    Button::new(
                        &mut self.tiny_button_state,
                        Text::new("Tiny button").size(Spacing::small_font_size()),
                    )
                    .on_press(())
                    .style(audio_style::Button::default()),
                )
                .padding([0, 0, Spacing::base_spacing(), 0])
                .into(),
                Container::new(
                    Button::new(
                        &mut self.chromeless_button_state,
                        Text::new("Chrome-less button"),
                    )
                    .on_press(())
                    .style(audio_style::ChromelessButton),
                )
                .padding([0, 0, Spacing::base_spacing(), 0])
                .into(),
            ]))
            .padding([Spacing::base_spacing(), 0])
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
        ])
        .into()
    }
}

fn dropdown_with_label(
    pick_list_state: &mut pick_list::State<String>,
    options: Vec<String>,
    selected_option: String,
) -> Element<Message> {
    Row::with_children(vec![
        Container::new(Text::new("Audio driver"))
            .width(Length::FillPortion(2))
            .align_x(alignment::Horizontal::Right)
            .center_y()
            .padding([0, Spacing::base_spacing()])
            .into(),
        Container::new(
            pick_list::PickList::new(pick_list_state, options, Some(selected_option), |option| {
                Message::Content(ContentMessage::PickList(option))
            })
            .style(audio_style::PickList)
            .padding(Spacing::base_spacing())
            .width(Length::Fill),
        )
        .width(Length::FillPortion(8))
        .into(),
    ])
    .width(Length::Fill)
    .align_items(Alignment::Center)
    .into()
}

struct Sidebar {
    menu_list: menu_list::State<String, ContentView>,
}

impl Sidebar {
    pub fn new() -> Self {
        Sidebar {
            menu_list: menu_list::State::new(
                vec![
                    (String::from("Tabs"), ContentView::Tabs),
                    (
                        String::from("Sliders and Knobs"),
                        ContentView::SlidersAndKnobs,
                    ),
                    (String::from("Buttons"), ContentView::Buttons),
                    (String::from("Dropdowns"), ContentView::Dropdowns),
                    (String::from("Tree view"), ContentView::TreeView),
                ],
                Some(0),
            ),
        }
    }

    fn update(&mut self, message: menu_list::Message<ContentView>) {
        self.menu_list.update(message.clone());
        match message {
            menu_list::Message::Selected { .. } => {}
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
                .style(audio_style::Container0::default())
                .into(),
        ])
        .into()
    }
}
