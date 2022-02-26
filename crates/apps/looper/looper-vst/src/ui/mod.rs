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
    looper_visualization: LooperVisualizationView<LooperVisualizationDrawModelImpl>,
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
                looper_visualization: LooperVisualizationView::new(
                    LooperVisualizationDrawModelImpl::new(
                        flags.processor_handle.clone(),
                        flags.sequencer_handle.clone(),
                    ),
                ),
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
        let status_message = if let Some((position_beats, tempo)) =
            time_info.position_beats().zip(time_info.tempo())
        {
            format!("{}bpm {:.1}", tempo, position_beats)
        } else {
            "Free tempo".into()
        };

        Column::with_children(vec![
            self.tabs_view
                .view(vec![
                    tabs::Tab::new(
                        "File Editor",
                        self.audio_editor_view.view().map(|_msg| Message::None),
                    ),
                    tabs::Tab::new(
                        "Main",
                        Container::new(Column::with_children(vec![
                            Container::new(
                                Text::new(status_message).size(Spacing::small_font_size()),
                            )
                            .padding(Spacing::base_spacing())
                            .style(Container0::default())
                            .width(Length::Fill)
                            .into(),
                            Container::new(
                                Container::new(
                                    self.looper_visualization.view().map(|_| Message::None),
                                )
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .style(Container0::default().border_width(1.)),
                            )
                            .padding(Spacing::base_spacing())
                            .style(Container1::default())
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .into(),
                            self.knobs_view.view().map(Message::BottomPanel),
                        ]))
                        .center_x()
                        .center_y()
                        .style(ContainerStyle)
                        .width(Length::Fill)
                        .height(Length::Fill),
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
                    bottom_panel::Message::ClearPressed => {
                        self.looper_visualization.clear_visualization()
                    }
                    _ => {}
                }

                self.knobs_view
                    .update(message)
                    .map(Message::BottomPanel)
                    .map(WrapperMessage::Inner)
            }
            Message::VisualizationTick => {
                self.looper_visualization.tick_visualization();
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
