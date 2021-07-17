use audio_garbage_collector::Shared;
use audio_processor_iced_design_system::colors::Colors;
use audio_processor_iced_design_system::spacing::Spacing;
use iced::canvas::{Cursor, Frame, Geometry, Program};
use iced::widget::canvas::Fill;
use iced::{Canvas, Container, Element, Length, Point, Rectangle, Size};
use plugin_host_lib::processors::volume_meter_processor::VolumeMeterProcessorHandle;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct VolumeInfo {
    left: f32,
    right: f32,
}

impl Default for VolumeInfo {
    fn default() -> Self {
        Self {
            left: 0.0,
            right: 0.0,
        }
    }
}

impl From<&Option<Shared<VolumeMeterProcessorHandle>>> for VolumeInfo {
    fn from(handle: &Option<Shared<VolumeMeterProcessorHandle>>) -> Self {
        match handle {
            None => VolumeInfo::default(),
            Some(handle) => VolumeInfo {
                left: handle.volume_left.get(),
                right: handle.volume_right.get(),
            },
        }
    }
}

type Message = ();

pub struct VolumeMeter {
    volume_info: VolumeInfo,
}

impl VolumeMeter {
    pub fn new(volume_info: VolumeInfo) -> Self {
        Self { volume_info }
    }

    pub fn view<'a>(self) -> Element<'a, ()> {
        Container::new(
            Canvas::new(VolumeMeterProgram::new(self.volume_info))
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(Spacing::medium_spacing())
        .into()
    }
}

struct VolumeMeterProgram {
    volume: VolumeInfo,
}

impl VolumeMeterProgram {
    pub fn new(volume: VolumeInfo) -> Self {
        VolumeMeterProgram { volume }
    }
}

impl Program<Message> for VolumeMeterProgram {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size());

        let spacing = Spacing::small_spacing() as f32 / 2.;
        let bar_width = bounds.width / 2. - spacing / 2.;
        VolumeMeterProgram::draw_volume_bar(&mut frame, self.volume.left, bar_width, 0.0);
        VolumeMeterProgram::draw_volume_bar(
            &mut frame,
            self.volume.right,
            bar_width,
            bar_width + spacing,
        );

        vec![frame.into_geometry()]
    }
}

impl VolumeMeterProgram {
    /// Draw a rectangle for volume
    fn draw_volume_bar(frame: &mut Frame, volume: f32, bar_width: f32, offset_x: f32) {
        let bar_height = volume * frame.height() * 10.;
        let y_coord = frame.height() - bar_height;
        // Background
        frame.fill_rectangle(
            Point::new(offset_x, 0.0),
            Size::new(bar_width, frame.height()),
            Fill::from(Colors::background_level0()),
        );
        // Volume
        frame.fill_rectangle(
            Point::new(offset_x, y_coord),
            Size::new(bar_width, bar_height),
            Fill::from(Colors::success()),
        );
    }
}
