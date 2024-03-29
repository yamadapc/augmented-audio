// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

use iced::widget::pane_grid::Axis;
use iced::{
    alignment, widget, widget::pick_list, widget::Button, widget::Column, widget::Container,
    widget::PaneGrid, widget::Row, widget::Rule, widget::Text, Alignment, Application, Command,
    Element, Length, Settings,
};
use iced_audio::{Normal, NormalParam};
use iced_style::Theme;
use widget::pane_grid;

use audio_processor_iced_design_system::container::HoverContainer;
use audio_processor_iced_design_system::knob::Knob;
use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style as audio_style;
use audio_processor_iced_design_system::tabs;
use audio_processor_iced_design_system::{knob as audio_knob, menu_list, tree_view};

fn main() -> iced::Result {
    wisual_logger::init_from_env();
    log::info!("Initializing app");
    WalkthroughApp::run(Settings {
        antialiasing: true,
        default_text_size: Spacing::default_font_size() as f32,
        ..Settings::default()
    })
}

#[derive(Debug, Clone)]
enum Message {
    PaneResized(pane_grid::ResizeEvent),
    KnobChanged(usize, Normal),
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
    type Theme = Theme;
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
            Message::KnobChanged(knob_id, value) => {
                self.pane_state.iter_mut().for_each(|(_, pane_state)| {
                    if let PaneState::Content(content) = pane_state {
                        if let Some(state) = content.knobs_view.knob_states.get_mut(knob_id) {
                            state.update(value);
                        }
                        content.knobs_view.sliders.state[knob_id].update(value);
                    }
                })
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let panel = PaneGrid::new(&self.pane_state, |_pane, state, _| match state {
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

    fn view(&self) -> Element<Message> {
        let content = self.content_view();

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .padding(Spacing::base_spacing())
            .style(audio_style::Container1::default())
            .into()
    }

    fn content_view(&self) -> Element<Message> {
        match self.content_view {
            ContentView::Dropdowns => {
                let options = vec![String::from("Option 1"), String::from("Option 2")];
                let selected_option = options[self.selected_option].clone();
                Column::with_children(vec![
                    Container::new(dropdown_with_label(
                        options.clone(),
                        selected_option.clone(),
                    ))
                    .padding([Spacing::base_spacing(), 0])
                    .into(),
                    Container::new(dropdown_with_label(
                        options.clone(),
                        selected_option.clone(),
                    ))
                    .padding([Spacing::base_spacing(), 0])
                    .into(),
                    Container::new(dropdown_with_label(options, selected_option))
                        .padding([Spacing::base_spacing(), 0])
                        .into(),
                ])
                .into()
            }
            ContentView::SlidersAndKnobs => self.knobs_view.view().into(),
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
    knob_states: Vec<iced_audio::NormalParam>,
    sliders: SlidersView,
}

impl KnobsView {
    pub fn new() -> Self {
        KnobsView {
            knob_states: vec![
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            ],
            sliders: SlidersView::new(),
        }
    }
}

impl KnobsView {
    fn view(&self) -> Element<Message> {
        let knobs = self
            .knob_states
            .iter()
            .enumerate()
            .map(|(knob_id, knob_state)| {
                HoverContainer::new(
                    Column::with_children(vec![
                        Text::new("Dry/Wet").size(Spacing::small_font_size()).into(),
                        Knob::new(*knob_state, move |value| {
                            Message::KnobChanged(knob_id, value)
                        })
                        .style(audio_knob::style::Knob)
                        .size(Length::Fixed(Spacing::base_control_size() as f32))
                        .into(),
                        Text::new(format!("{:.0}%", knob_state.value.as_f32() * 100.0))
                            .size(Spacing::small_font_size())
                            .into(),
                    ])
                    .align_items(Alignment::Center)
                    .spacing(Spacing::small_spacing()),
                )
                .style(audio_style::HoverContainer::default())
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
    state: Vec<NormalParam>,
}

impl SlidersView {
    pub fn new() -> Self {
        SlidersView {
            state: (0..10).map(|_| NormalParam::default()).collect(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let elements = self
            .state
            .iter()
            .enumerate()
            .map(|(id, state)| {
                iced_audio::VSlider::new(*state, move |value| Message::KnobChanged(id, value))
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

struct ButtonsView {}

impl ButtonsView {
    pub fn new() -> Self {
        ButtonsView {}
    }
}

impl ButtonsView {
    fn view(&self) -> Element<()> {
        Column::with_children(vec![
            Container::new(Text::new("Buttons"))
                .padding([0, 0, Spacing::base_spacing(), 0])
                .into(),
            Rule::horizontal(1).style(audio_style::Rule).into(),
            Container::new(Column::with_children(vec![
                Container::new(
                    Button::new(Text::new("Default button"))
                        .on_press(())
                        .style(audio_style::Button::default().into()),
                )
                .padding([0, 0, Spacing::base_spacing(), 0])
                .into(),
                Container::new(
                    Button::new(Text::new("Tiny button").size(Spacing::small_font_size()))
                        .on_press(())
                        .style(audio_style::Button::default().into()),
                )
                .padding([0, 0, Spacing::base_spacing(), 0])
                .into(),
                Container::new(
                    Button::new(Text::new("Chrome-less button"))
                        .on_press(())
                        .style(audio_style::ChromelessButton.into()),
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

fn dropdown_with_label(options: Vec<String>, selected_option: String) -> Element<'static, Message> {
    Row::with_children(vec![
        Container::new(Text::new("Audio driver"))
            .width(Length::FillPortion(2))
            .align_x(alignment::Horizontal::Right)
            .center_y()
            .padding([0, Spacing::base_spacing()])
            .into(),
        Container::new(
            pick_list::PickList::new(options, Some(selected_option), |option| {
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

    fn view(&self) -> Element<Message> {
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
