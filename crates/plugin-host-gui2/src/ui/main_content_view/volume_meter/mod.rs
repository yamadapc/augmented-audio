use std::sync::Arc;
use std::sync::Mutex;

use iced::canvas::event::Status;
use iced::canvas::{Cursor, Event, Frame, Geometry, Program, Stroke};
use iced::mouse::Interaction;
use iced::widget::canvas::Fill;
use iced::{Canvas, Command, Container, Element, Length, Point, Rectangle, Size, Vector};

use audio_garbage_collector::Shared;
use audio_processor_iced_design_system::colors::Colors;
use audio_processor_iced_design_system::spacing::Spacing;
use plugin_host_lib::processors::volume_meter_processor::VolumeMeterProcessorHandle;
use plugin_host_lib::TestPluginHost;

// TODO - this whole file needs to be refactored

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

#[derive(Clone, Debug)]
pub enum Message {
    DragStart,
    VolumeChange { delta: f32 },
    DragEnd,
    None,
}

pub struct State {
    volume: f32,
    mouse_state: MouseState,
}

impl Default for State {
    fn default() -> Self {
        Self {
            volume: 1.0,
            mouse_state: Default::default(),
        }
    }
}

#[derive(Default)]
struct MouseState {
    dragging: Option<Point>,
}

pub struct VolumeMeter {
    volume_info: VolumeInfo,
}

impl VolumeMeter {
    pub fn new(volume_info: VolumeInfo) -> Self {
        Self { volume_info }
    }

    pub fn view(self, state: &mut State) -> Element<Message> {
        Container::new(
            Canvas::new(VolumeMeterProgram::new(self.volume_info, state))
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(Spacing::medium_spacing())
        .into()
    }
}

pub fn update(
    state: &mut State,
    message: Message,
    plugin_host: Arc<Mutex<TestPluginHost>>,
    volume_handle: &mut Option<Shared<VolumeMeterProcessorHandle>>,
) -> Command<Message> {
    match message {
        Message::VolumeChange { delta } => Command::perform(
            async move {
                let mut plugin_host = plugin_host.lock().unwrap();
                plugin_host.set_volume(delta);
            },
            |_| Message::None,
        ),
        _ => Command::none(),
    }
}

struct VolumeMeterProgram<'a> {
    volume: VolumeInfo,
    state: &'a mut State,
}

impl<'a> VolumeMeterProgram<'a> {
    pub fn new(volume: VolumeInfo, state: &'a mut State) -> Self {
        VolumeMeterProgram { volume, state }
    }

    /// True if the cursor is currently dragging the volume meter handle
    fn is_dragging(&self) -> bool {
        self.state.mouse_state.dragging.is_some()
    }
}

impl<'a> Program<Message> for VolumeMeterProgram<'a> {
    fn update(
        &mut self,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (Status, Option<Message>) {
        let ignore = (iced::canvas::event::Status::Ignored, None);
        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                iced::mouse::Event::CursorMoved { position } => {
                    if let Some(last_position) = self.state.mouse_state.dragging {
                        let top_left_position = bounds.y;
                        let relative_y =
                            (bounds.height - (position.y - top_left_position)) / bounds.height;
                        let volume = VolumeMeterProgram::y_perc_to_amplitude(relative_y)
                            .min(3.0)
                            .max(0.0);
                        self.state.volume = volume;
                        log::info!("relative_y={} volume={}", relative_y, volume);
                        (
                            iced::canvas::event::Status::Captured,
                            Some(Message::VolumeChange { delta: volume }),
                        )
                    } else {
                        ignore
                    }
                }
                iced::mouse::Event::ButtonPressed(_) => {
                    if cursor.is_over(&bounds) {
                        self.state.mouse_state.dragging = cursor.position();
                        (
                            iced::canvas::event::Status::Captured,
                            Some(Message::DragStart),
                        )
                    } else {
                        ignore
                    }
                }
                iced::mouse::Event::ButtonReleased(_) => {
                    let was_dragging = self.is_dragging();
                    self.state.mouse_state.dragging = None;
                    if was_dragging {
                        (
                            iced::canvas::event::Status::Captured,
                            Some(Message::DragEnd),
                        )
                    } else {
                        ignore
                    }
                }
                _ => ignore,
            },
            _ => ignore,
        }
    }

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
        VolumeMeterProgram::draw_volume_handle(
            &mut frame,
            bar_width * 2. + spacing,
            self.state.volume,
        );

        vec![frame.into_geometry()]
    }

    fn mouse_interaction(&self, bounds: Rectangle, cursor: Cursor) -> Interaction {
        if self.is_dragging() || cursor.is_over(&bounds) {
            iced::mouse::Interaction::ResizingVertically
        } else {
            iced::mouse::Interaction::default()
        }
    }
}

impl<'a> VolumeMeterProgram<'a> {
    /// Draw a rectangle for volume
    fn draw_volume_bar(
        frame: &mut Frame,
        volume: f32,
        peak_volume: f32,
        bar_width: f32,
        offset_x: f32,
    ) {
        let max_ampl = db_to_render(2.0);
        let min_ampl = db_to_render(-144.0);
        let bar_height = interpolate(volume, (min_ampl, max_ampl), (0.0, frame.height()));
        let peak_bar_height = interpolate(peak_volume, (min_ampl, max_ampl), (0.0, frame.height()));

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

    /// Draw the volume handle
    fn draw_volume_handle(frame: &mut Frame, offset_x: f32, volume: f32) {
        let mut handle_path = iced::canvas::path::Builder::new();
        let handle_width = 10.0;

        let start_x = offset_x - handle_width / 2.;
        let tick_y = VolumeMeterProgram::amplitude_y_position(frame, volume)
            .max(0.0)
            .min(frame.height());
        let start_point = Point::new(start_x, tick_y);

        handle_path.move_to(start_point);
        handle_path.line_to(start_point + Vector::new(handle_width, handle_width / 2.0));
        handle_path.line_to(start_point + Vector::new(handle_width, -handle_width / 2.0));
        handle_path.line_to(start_point);

        frame.fill(
            &handle_path.build(),
            Fill::from(Colors::border_color().darken(-0.5)),
        );
    }

    /// Draw a mark and text reference for the `value` decibels point.
    ///
    /// TODO - This should be cached between render passes.
    fn draw_mark(frame: &mut Frame, bar_width: f32, offset_x: f32, value: f32) {
        let text = format!("{:>4.0}", value);
        let tick_y = VolumeMeterProgram::decibels_y_position(frame, value);
        let mut tick_path = iced::canvas::path::Builder::new();
        tick_path.move_to(Point::new(offset_x, tick_y));
        tick_path.line_to(Point::new(offset_x + bar_width, tick_y));
        frame.stroke(
            &tick_path.build(),
            Stroke::default().with_color(Colors::border_color()),
        );
        frame.translate(Vector::new(bar_width * 2. + 5., tick_y - 8.0));
        frame.fill_text(iced::canvas::Text {
            content: text,
            color: Colors::border_color(),
            ..iced::canvas::Text::default()
        });
        frame.translate(Vector::new(-(bar_width * 2. + 5.), -(tick_y - 8.0)));
    }

    // TODO - this is just wrong
    fn y_perc_to_amplitude(y_perc: f32) -> f32 {
        y_perc
    }

    fn amplitude_y_position(frame: &mut Frame, value: f32) -> f32 {
        // let max_ampl = db_to_render(2.0);
        // let min_ampl = db_to_render(-144.0);
        interpolate(value, (0.0, 1.0), (frame.height(), 0.0))
    }

    /// The Y coordinate for a given `value` in decibels. The return value is reversed.
    ///
    /// This is for rendering values between -Infinity decibels and `VolumeMeterProgram::max_amplitude`
    /// decibels.
    fn decibels_y_position(frame: &mut Frame, value: f32) -> f32 {
        let max_ampl = VolumeMeterProgram::max_amplitude();
        let min_ampl = VolumeMeterProgram::min_amplitude();
        let value = db_to_render(value);
        interpolate(value, (min_ampl, max_ampl), (frame.height(), 0.0))
    }

    /// Maximum amplitude rendered at the top-most Y coordinate
    fn max_amplitude() -> f32 {
        db_to_render(2.0)
    }

    /// Minimum amplitude rendered at the bottom-most Y coordinate
    fn min_amplitude() -> f32 {
        db_to_render(-144.0)
    }
}

/// Convert a number in decibels to the rendering range.
/// This has no meaning other than to be a logarithmic scaled float that fits nicely within the UI.
fn db_to_render(db: f32) -> f32 {
    let reference_amplitude = 1e-1;
    (10.0_f32).powf(db / 60.0) * reference_amplitude
}

/// Convert decibels to amplitude
#[allow(dead_code)]
fn db_to_amplitude(db: f32) -> f32 {
    let reference_amplitude = 1e-10;
    (10.0_f32).powf(db / 20.0) * reference_amplitude
}

/// Convert amplitude to decibels
#[allow(dead_code)]
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
