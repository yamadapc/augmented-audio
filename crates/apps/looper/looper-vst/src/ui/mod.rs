use std::time::Duration;

use iced::{Column, Container, Length, Text};
use iced_baseview::{executor, Subscription, WindowSubs};
use iced_baseview::{Application, Command, Element};
use vst::host::Host;
use vst::plugin::HostCallback;

use audio_garbage_collector::Shared;
use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style::Container0;
use audio_processor_iced_design_system::style::Container1;
use audio_processor_iced_design_system::tabs;
use looper_processor::{LoopSequencerProcessorHandle, LooperProcessorHandle};
use looper_visualization::LooperVisualizationView;
use style::ContainerStyle;

use crate::ui::looper_visualization::LooperVisualizationDrawModelImpl;

mod bottom_panel;
mod looper_visualization;
mod style;

#[derive(Clone)]
pub struct Flags {
    pub host_callback: Option<HostCallback>,
    pub processor_handle: Shared<LooperProcessorHandle>,
    pub sequencer_handle: Shared<LoopSequencerProcessorHandle>,
}

pub struct LooperApplication {
    // processor_handle: Shared<LooperProcessorHandle>,
    host_callback: Option<HostCallback>,
    looper_visualization: LooperVisualizationView<LooperVisualizationDrawModelImpl>,
    knobs_view: bottom_panel::BottomPanelView,
    tabs_view: tabs::State,
}

#[derive(Clone, Debug)]
pub enum Message {
    BottomPanel(bottom_panel::Message),
    VisualizationTick,
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
                // processor_handle: flags.processor_handle.clone(),
                host_callback: flags.host_callback,
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
        iced::time::every(Duration::from_millis(64))
            .map(|_| WrapperMessage::Inner(Message::VisualizationTick))
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let time_info = self
            .host_callback
            .map(|cb| {
                cb.get_time_info(
                    (vst::api::TimeInfoFlags::TEMPO_VALID | vst::api::TimeInfoFlags::PPQ_POS_VALID)
                        .bits(),
                )
            })
            .flatten();
        let status_message = if let Some(time_info) = time_info {
            format!("{}bpm {:.1}", time_info.tempo, time_info.ppq_pos,)
            // format!(
            //     "{}bpm {:.1} / {}",
            //     time_info.tempo,
            //     time_info.ppq_pos,
            //     &*self.processor_handle.debug()
            // )
        } else {
            "Free tempo".into()
        };

        self.tabs_view
            .view(vec![
                tabs::Tab::new(
                    "Main",
                    Container::new(Column::with_children(vec![
                        Container::new(Text::new(status_message).size(Spacing::small_font_size()))
                            .padding(Spacing::base_spacing())
                            .style(Container0::default())
                            .width(Length::Fill)
                            .into(),
                        Container::new(
                            Container::new(self.looper_visualization.view().map(|_| Message::None))
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
                // tabs::Tab::new(
                //     "Settings",
                //     Container::new(Text::new("Settings"))
                //         .center_x()
                //         .center_y()
                //         .style(ContainerStyle)
                //         .width(Length::Fill)
                //         .height(Length::Fill),
                // ),
            ])
            .map(|msg| WrapperMessage::TabsView(msg))
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
            Message::None => Command::none(),
        }
    }
}
