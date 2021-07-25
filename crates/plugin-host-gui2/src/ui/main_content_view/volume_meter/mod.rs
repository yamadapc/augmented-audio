use audio_garbage_collector::Shared;
use audio_processor_iced_design_system::colors::Colors;
use audio_processor_iced_design_system::spacing::Spacing;
use iced::canvas::{Cursor, Frame, Geometry, Program, Stroke};
use iced::widget::canvas::Fill;
use iced::{Canvas, Container, Element, Length, Point, Rectangle, Size, Vector};
use plugin_host_lib::processors::volume_meter_processor::VolumeMeterProcessorHandle;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct VolumeInfo {
    left: f32,
    right: f32,
    left_peak: f32,
    right_peak: f32,
}

impl Default for VolumeInfo {
    fn default() -> Self {
        Self {
            left: 0.0,
            right: 0.0,
            left_peak: 0.0,
            right_peak: 0.0,
        }
    }
}

impl From<&Option<Shared<VolumeMeterProcessorHandle>>> for VolumeInfo {
    fn from(handle: &Option<Shared<VolumeMeterProcessorHandle>>) -> Self {
        match handle {
            None => VolumeInfo::default(),
            Some(handle) => VolumeInfo {
                left: handle.volume_left.get(),
                left_peak: handle.peak_left.get(),
                right: handle.volume_right.get(),
                right_peak: handle.peak_right.get(),
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
        let bar_width = bounds.width / 4. - spacing / 2.;
        VolumeMeterProgram::draw_volume_bar(
            &mut frame,
            self.volume.left,
            self.volume.left_peak,
            bar_width,
            0.0,
        );
        VolumeMeterProgram::draw_volume_bar(
            &mut frame,
            self.volume.right,
            self.volume.right_peak,
            bar_width,
            bar_width + spacing,
        );

        vec![frame.into_geometry()]
    }
}

impl VolumeMeterProgram {
    /// Draw a rectangle for volume
    fn draw_volume_bar(
        frame: &mut Frame,
        volume: f32,
        peak_volume: f32,
        bar_width: f32,
        offset_x: f32,
    ) {
        // let volume = amplitude_to_db(volume).max(-144.);
        // let peak_volume = (20.0 * (peak_volume / reference_amplitude).log10()).max(-144.);

        let max_ampl = db_to_render(2.0);
        let min_ampl = db_to_render(-144.0);
        let bar_height = interpolate(volume, (min_ampl, max_ampl), (0.0, frame.height()));
        let peak_bar_height = interpolate(peak_volume, (min_ampl, max_ampl), (0.0, frame.height()));

        // let bar_height = interpolate(volume, (-144.0, 6.0), (0.0, frame.height()));
        // let peak_bar_height = interpolate(peak_volume, (-144.0, 6.0), (0.0, frame.height()));
        log::debug!(
            "Drawing volume volume={} peak_volume={} / bar_height={} peak_bar_height={}",
            volume,
            peak_volume,
            bar_height,
            peak_bar_height
        );

        let y_coord = frame.height() - bar_height;
        let peak_y_coord = frame.height() - peak_bar_height;
        // Background
        frame.fill_rectangle(
            Point::new(offset_x, 0.0),
            Size::new(bar_width, frame.height()),
            Fill::from(Colors::background_level0()),
        );

        // Marks
        let marks = [0.0, -12.0, -24.0, -36.0, -48.0, -60.0];
        for value in marks {
            VolumeMeterProgram::draw_mark(frame, bar_width, offset_x, value);
        }

        // Peak Volume
        frame.fill_rectangle(
            Point::new(offset_x, peak_y_coord),
            Size::new(bar_width, peak_bar_height),
            Fill::from(Colors::success().darken(0.4)),
        );
        // RMS Volume
        frame.fill_rectangle(
            Point::new(offset_x, y_coord),
            Size::new(bar_width, bar_height),
            Fill::from(Colors::success()),
        );
    }

    fn draw_mark(frame: &mut Frame, bar_width: f32, offset_x: f32, value: f32) {
        let text = format!("{:.0}", value);
        let max_ampl = db_to_render(2.0);
        let min_ampl = db_to_render(-144.0);
        let value = db_to_render(value);
        let tick_y = interpolate(value, (min_ampl, max_ampl), (frame.height(), 0.0));
        let mut tick_path = iced::canvas::path::Builder::new();
        tick_path.move_to(Point::new(offset_x, tick_y));
        tick_path.line_to(Point::new(offset_x + bar_width, tick_y));
        frame.stroke(
            &tick_path.build(),
            Stroke::default().with_color(Colors::border_color()),
        );
        frame.translate(Vector::new(bar_width * 2. + 5., tick_y - 10.0));
        frame.fill_text(iced::canvas::Text {
            content: text,
            color: Colors::text(),
            ..iced::canvas::Text::default()
        });
        frame.translate(Vector::new(-(bar_width * 2. + 5.), -(tick_y - 10.0)));
    }
}

fn db_to_render(db: f32) -> f32 {
    let reference_amplitude = 1e-1;
    (10.0_f32).powf(db / 60.0) * reference_amplitude
}

fn db_to_amplitude(db: f32) -> f32 {
    let reference_amplitude = 1e-10;
    (10.0_f32).powf(db / 20.0) * reference_amplitude
}

fn amplitude_to_db(volume: f32) -> f32 {
    let reference_amplitude = 1e-10;
    20.0 * (volume / reference_amplitude).log10()
}

fn interpolate(value: f32, range_from: (f32, f32), range_to: (f32, f32)) -> f32 {
    let bounds_from = range_from.1 - range_from.0;
    let bounds_to = range_to.1 - range_to.0;
    range_to.0 + (value - range_from.0) / bounds_from * bounds_to
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_interpolate() {
        assert_eq!(interpolate(1., (0., 1.), (0., 2.)), 2.);
    }

    #[test]
    fn test_interpolate_negative_range() {
        assert_eq!(interpolate(0., (-1., 1.), (0., 2.)), 1.);
    }

    #[test]
    fn test_interpolate_reversed_range() {
        assert_eq!(interpolate(0., (-1., 1.), (2., 0.)), 1.);
    }
}
