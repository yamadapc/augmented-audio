use iced::{Align, Background, Color, Column, Container, Length, Row, Rule, Text};

use audio_garbage_collector::Shared;
use audio_processor_iced_design_system::colors::Colors;
use audio_processor_iced_design_system::container::HoverContainer;
use audio_processor_iced_design_system::knob as audio_knob;
use audio_processor_iced_design_system::knob::Knob;
use audio_processor_iced_design_system::spacing::Spacing;
use audio_processor_iced_design_system::style as audio_style;
use audio_processor_iced_design_system::style::Container0;
use audio_processor_iced_design_system::style::Container1;
use iced_baseview::canvas::{Cursor, Geometry, Program};
use iced_baseview::container::Style;
use iced_baseview::{executor, Canvas, Rectangle, Subscription, WindowSubs};
use iced_baseview::{Application, Command, Element};
use looper_processor::LooperProcessorHandle;
use looper_visualization::LooperVisualizationView;
use std::time::Duration;
use style::ContainerStyle;

mod bottom_panel;
mod looper_visualization;
mod style;

#[derive(Clone)]
pub struct Flags {
    pub processor_handle: Shared<LooperProcessorHandle<f32>>,
}

pub struct LooperApplication {
    processor_handle: Shared<LooperProcessorHandle<f32>>,
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
                looper_visualization: LooperVisualizationView::new(flags.processor_handle),
                knobs_view: bottom_panel::BottomPanelView::new(),
            },
            Command::none(),
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::BottomPanel(message) => {
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
        iced::time::every(Duration::from_millis(100)).map(|_| Message::VisualizationTick)
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
