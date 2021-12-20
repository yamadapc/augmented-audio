use std::time::Duration;

use iced::{Column, Container, Length};

use audio_garbage_collector::Shared;
use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style::Container0;
use audio_processor_iced_design_system::style::Container1;
use iced_baseview::{executor, Subscription, WindowSubs};
use iced_baseview::{Application, Command, Element};
use looper_processor::LooperProcessorHandle;
use looper_visualization::LooperVisualizationView;
use style::ContainerStyle;

mod bottom_panel;
mod looper_visualization;
mod style;

#[derive(Clone)]
pub struct Flags {
    pub processor_handle: Shared<LooperProcessorHandle>,
}

pub struct LooperApplication {
    #[allow(dead_code)]
    processor_handle: Shared<LooperProcessorHandle>,
    looper_visualization: LooperVisualizationView,
    knobs_view: bottom_panel::BottomPanelView,
}

#[derive(Clone, Debug)]
pub enum Message {
    BottomPanel(bottom_panel::Message),
    VisualizationTick,
    None,
}

impl Application for LooperApplication {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            LooperApplication {
                processor_handle: flags.processor_handle.clone(),
                looper_visualization: LooperVisualizationView::new(flags.processor_handle.clone()),
                knobs_view: bottom_panel::BottomPanelView::new(flags.processor_handle),
            },
            Command::none(),
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::BottomPanel(message) => {
                match &message {
                    bottom_panel::Message::ClearPressed => {
                        self.looper_visualization.clear_visualization()
                    }
                    _ => {}
                }

                self.knobs_view.update(message).map(Message::BottomPanel)
            }
            Message::VisualizationTick => {
                self.looper_visualization.tick_visualization();
                Command::none()
            }
            Message::None => Command::none(),
        }
    }

    fn subscription(
        &self,
        _window_subs: &mut WindowSubs<Self::Message>,
    ) -> Subscription<Self::Message> {
        iced::time::every(Duration::from_millis(16)).map(|_| Message::VisualizationTick)
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Container::new(Column::with_children(vec![
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
        .height(Length::Fill)
        .into()
    }
}
