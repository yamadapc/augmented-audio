use std::path::PathBuf;
use std::time::Duration;

use iced::{Column, Container, Length, Text};
use iced_baseview::{executor, Subscription, WindowSubs};
use iced_baseview::{Application, Command, Element};

use audio_garbage_collector::Shared;
use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style::Container0;
use audio_processor_iced_design_system::style::Container1;
use audio_processor_iced_design_system::tabs;
use looper_processor::{LoopSequencerProcessorHandle, LooperProcessorHandle, TimeInfoProvider};
use looper_visualization::LooperVisualizationView;
use style::ContainerStyle;

use crate::services::audio_file_manager::AudioFileManager;
use crate::ui::audio_editor_view::AudioEditorView;
use crate::ui::looper_visualization::LooperVisualizationDrawModelImpl;

pub mod audio_editor_view;
mod bottom_panel;
mod common;
mod file_drag_and_drop_handler;
mod looper_visualization;
mod style;

#[derive(Clone)]
pub struct Flags {
    pub processor_handle: Shared<LooperProcessorHandle>,
    pub sequencer_handle: Shared<LoopSequencerProcessorHandle>,
}

pub struct LooperApplication {
    processor_handle: Shared<LooperProcessorHandle>,
    looper_visualizations: Vec<LooperVisualizationView>,
    knobs_view: bottom_panel::BottomPanelView,
    audio_file_manager: AudioFileManager,
    audio_editor_view: AudioEditorView,
    tabs_view: tabs::State,
}

#[derive(Clone, Debug)]
pub enum Message {
    BottomPanel(bottom_panel::Message),
    VisualizationTick,
    FileHover(PathBuf),
    FileDropped(PathBuf),
    None,
}

#[derive(Clone, Debug)]
pub enum WrapperMessage {
    Inner(Message),
    TabsView(tabs::Message<Message>),
}

impl Application for LooperApplication {
    type Executor = executor::Default;
    type Message = WrapperMessage;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            LooperApplication {
                processor_handle: flags.processor_handle.clone(),
                audio_file_manager: AudioFileManager::new(),
                audio_editor_view: AudioEditorView::default(),
                looper_visualizations: vec![LooperVisualizationView::new(
                    LooperVisualizationDrawModelImpl::new(
                        flags.processor_handle.clone(),
                        flags.sequencer_handle.clone(),
                    ),
                )],
                knobs_view: bottom_panel::BottomPanelView::new(
                    flags.processor_handle,
                    flags.sequencer_handle,
                ),
                tabs_view: tabs::State::new(),
            },
            Command::none(),
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            WrapperMessage::Inner(message) => self.update_inner(message),
            WrapperMessage::TabsView(tabs::Message::Inner(message)) => self.update_inner(message),
            WrapperMessage::TabsView(message) => {
                self.tabs_view.update(message);
                Command::none()
            }
        }
    }

    fn subscription(
        &self,
        _window_subs: &mut WindowSubs<Self::Message>,
    ) -> Subscription<Self::Message> {
        Subscription::batch([iced::time::every(Duration::from_millis(64))
            .map(|_| WrapperMessage::Inner(Message::VisualizationTick))])
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let time_info_provider = self.processor_handle.time_info_provider();
        let time_info = time_info_provider.get_time_info();
        let status_bar = status_bar(time_info);

        let looper_views = Column::with_children(
            self.looper_visualizations
                .iter_mut()
                .map(|visualization| {
                    let inner: Element<'_, Message> = visualization.view().map(|_| Message::None);

                    Container::new(
                        Container::new(inner)
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .style(Container0::default().border_width(1.)),
                    )
                    .padding(Spacing::base_spacing())
                    .style(Container1::default())
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
                })
                .collect(),
        )
        .height(Length::Fill)
        .into();
        let knobs = Container::new(self.knobs_view.view().map(Message::BottomPanel))
            .height(Length::Units(100))
            .into();

        Column::with_children(vec![
            self.tabs_view
                .view(vec![
                    tabs::Tab::new(
                        "Main",
                        Container::new(Column::with_children(vec![
                            status_bar,
                            looper_views,
                            knobs,
                        ]))
                        .center_x()
                        .center_y()
                        .style(ContainerStyle)
                        .width(Length::Fill)
                        .height(Length::Fill),
                    ),
                    tabs::Tab::new(
                        "File Editor",
                        self.audio_editor_view.view().map(|_msg| Message::None),
                    ),
                ])
                .map(|msg| WrapperMessage::TabsView(msg))
                .into(),
            file_drag_and_drop_handler::FileDragAndDropHandler::new()
                .view()
                .map(WrapperMessage::Inner),
        ])
        .into()
    }
}

impl LooperApplication {
    fn update_inner(&mut self, message: Message) -> Command<WrapperMessage> {
        match message {
            Message::BottomPanel(message) => {
                match &message {
                    bottom_panel::Message::ClearPressed => self
                        .looper_visualizations
                        .iter_mut()
                        .for_each(|v| v.clear_visualization()),
                    _ => {}
                }

                self.knobs_view
                    .update(message)
                    .map(Message::BottomPanel)
                    .map(WrapperMessage::Inner)
            }
            Message::VisualizationTick => {
                self.looper_visualizations
                    .iter_mut()
                    .for_each(|v| v.tick_visualization());
                Command::none()
            }
            Message::FileDropped(path) => {
                let _file_id = self.audio_file_manager.read_file(path);
                Command::none()
            }
            Message::FileHover(_path) => Command::none(),
            Message::None => Command::none(),
        }
    }
}

fn status_bar<'a>(time_info: looper_processor::TimeInfo) -> Element<'a, Message> {
    let status_message = Text::new(
        if let Some((position_beats, tempo)) = time_info.position_beats().zip(time_info.tempo()) {
            format!("{}bpm {:.1}", tempo, position_beats)
        } else {
            "Free tempo".into()
        },
    )
    .size(Spacing::small_font_size());
    let status_bar = Container::new(status_message)
        .padding(Spacing::base_spacing())
        .style(Container0::default())
        .width(Length::Fill)
        .into();
    status_bar
}
