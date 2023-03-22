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
// use std::cell::RefCell;

use iced::mouse::Interaction;
use iced::widget::canvas::event::Status;
use iced::widget::canvas::Fill;
use iced::widget::canvas::{Cache, Cursor, Event, Frame, Geometry, Program, Stroke};
use iced::{
    widget::Canvas, widget::Container, Command, Element, Length, Point, Rectangle, Size, Vector,
};

use audio_garbage_collector::Shared;
use audio_processor_iced_design_system::colors::{darken_color, Colors};
use audio_processor_iced_design_system::spacing::Spacing;
use augmented_audio_volume::{Amplitude, Decibels};
use plugin_host_lib::audio_io::processor_handle_registry::ProcessorHandleRegistry;
use plugin_host_lib::processors::test_host_processor::TestHostProcessorHandle;
use plugin_host_lib::processors::volume_meter_processor::VolumeMeterProcessorHandle;

#[derive(Default, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct VolumeInfo {
    left: Amplitude,
    right: Amplitude,
    left_peak: Amplitude,
    right_peak: Amplitude,
}

impl From<&Option<Shared<VolumeMeterProcessorHandle>>> for VolumeInfo {
    fn from(handle: &Option<Shared<VolumeMeterProcessorHandle>>) -> Self {
        match handle {
            None => VolumeInfo::default(),
            Some(handle) => VolumeInfo {
                left: Amplitude::from(handle.volume_left.get()),
                left_peak: Amplitude::from(handle.peak_left.get()),
                right: Amplitude::from(handle.volume_right.get()),
                right_peak: Amplitude::from(handle.peak_right.get()),
            },
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    DragStart,
    VolumeChange { value: Decibels },
    DragEnd,
    None,
}

pub struct State {
    volume: Decibels,
    mouse_state: MouseState,
}

impl Default for State {
    fn default() -> Self {
        Self {
            volume: Decibels::from_db(0.0),
            mouse_state: Default::default(),
        }
    }
}

#[derive(Default)]
struct MouseState {
    dragging: bool,
}

pub struct VolumeMeter {
    volume_info: VolumeInfo,
    state: State,
    // frame: RefCell<Frame>,
    left_cache: Cache,
    right_cache: Cache,
}

impl Default for VolumeMeter {
    fn default() -> Self {
        Self::new()
    }
}

impl VolumeMeter {
    pub fn new() -> Self {
        Self {
            volume_info: VolumeInfo::default(),
            state: State::default(),
            // frame: RefCell::new(Frame::new(Size::new(100., 100.))),
            left_cache: Default::default(),
            right_cache: Default::default(),
        }
    }

    pub fn set_volume_info(&mut self, volume_info: VolumeInfo) {
        self.volume_info = volume_info;
    }

    pub fn set_volume_handle(&mut self, value: Decibels) {
        self.state.volume = value;
    }

    pub fn view(&self) -> Element<Message> {
        Container::new(Canvas::new(self).width(Length::Fill).height(Length::Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(Spacing::medium_spacing())
            .into()
    }

    /// True if the cursor is currently dragging the volume meter handle
    fn is_dragging(&self) -> bool {
        self.state.mouse_state.dragging
    }
}

pub fn update(message: Message) -> Command<Message> {
    match message {
        Message::VolumeChange { value: delta } => {
            let volume = delta.as_amplitude(1.0);
            let test_host_processor: Shared<TestHostProcessorHandle> =
                ProcessorHandleRegistry::current()
                    .get("test-host-processor")
                    .unwrap();
            test_host_processor.set_volume(volume);

            Command::none()
        }
        _ => Command::none(),
    }
}

impl Program<Message> for VolumeMeter {
    type State = State;

    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (Status, Option<Message>) {
        let ignore = (iced::widget::canvas::event::Status::Ignored, None);
        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                iced::mouse::Event::CursorMoved { position } => {
                    if state.mouse_state.dragging {
                        let top_left_position = bounds.y;
                        let relative_y =
                            (bounds.height - (position.y - top_left_position)) / bounds.height;
                        let volume = VolumeMeter::y_perc_to_db(relative_y);
                        state.volume = volume;
                        log::trace!(
                            "VolumeMeter::update relative_y={} volume={}",
                            relative_y,
                            volume.as_db()
                        );
                        (
                            iced::widget::canvas::event::Status::Captured,
                            Some(Message::VolumeChange {
                                value: state.volume,
                            }),
                        )
                    } else {
                        ignore
                    }
                }
                iced::mouse::Event::ButtonPressed(_) => {
                    if cursor.is_over(&bounds) {
                        state.mouse_state.dragging = true;
                        (
                            iced::widget::canvas::event::Status::Captured,
                            Some(Message::DragStart),
                        )
                    } else {
                        ignore
                    }
                }
                iced::mouse::Event::ButtonReleased(_) => {
                    let was_dragging = self.is_dragging();
                    state.mouse_state.dragging = false;
                    if was_dragging {
                        (
                            iced::widget::canvas::event::Status::Captured,
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

    fn draw(
        &self,
        _state: &Self::State,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(bounds.size()); // self.frame.borrow_mut();
                                                   // frame.resize(bounds.size());

        let spacing = Spacing::small_spacing() as f32 / 2.;
        let bar_width = bounds.width / 4. - spacing / 2.;

        // Draw background, the results will be cached
        let mut geometry = vec![
            Self::draw_volume_background(&self.left_cache, &bounds, bar_width, 0.0),
            Self::draw_volume_background(
                &self.right_cache,
                &bounds,
                bar_width,
                bar_width + spacing,
            ),
        ];

        // Draw volume bars
        Self::draw_volume_bar(
            &mut frame,
            self.volume_info.left,
            self.volume_info.left_peak,
            bar_width,
            0.0,
        );
        Self::draw_volume_bar(
            &mut frame,
            self.volume_info.right,
            self.volume_info.right_peak,
            bar_width,
            bar_width + spacing,
        );
        Self::draw_volume_handle(&mut frame, bar_width * 2. + spacing, self.state.volume);
        geometry.push(frame.into_geometry());

        geometry
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> Interaction {
        if self.is_dragging() || cursor.is_over(&bounds) {
            iced::mouse::Interaction::ResizingVertically
        } else {
            iced::mouse::Interaction::default()
        }
    }
}

impl VolumeMeter {
    fn draw_volume_background(
        cache: &Cache,
        bounds: &Rectangle,
        bar_width: f32,
        offset_x: f32,
    ) -> Geometry {
        cache.draw(bounds.size(), |frame| {
            // Background
            frame.fill_rectangle(
                Point::new(offset_x, 0.0),
                Size::new(bar_width, frame.height()),
                Fill::from(Colors::background_level0()),
            );

            // Marks
            let marks = [0.0, -12.0, -24.0, -36.0, -48.0, -60.0];
            for value in marks {
                Self::draw_mark(frame, bar_width, offset_x, value.into());
            }
        })
    }

    /// Draw a rectangle for volume
    fn draw_volume_bar(
        frame: &mut Frame,
        volume: Amplitude,
        peak_volume: Amplitude,
        bar_width: f32,
        offset_x: f32,
    ) {
        let max_ampl = db_to_render(2.0);
        let min_ampl = db_to_render(-144.0);
        let bar_height = interpolate(
            volume.as_amplitude(),
            (min_ampl, max_ampl),
            (0.0, frame.height()),
        );
        let peak_bar_height = interpolate(
            peak_volume.as_amplitude(),
            (min_ampl, max_ampl),
            (0.0, frame.height()),
        );

        log::trace!(
            "Drawing volume volume={} peak_volume={} / bar_height={} peak_bar_height={}",
            volume.as_amplitude(),
            peak_volume.as_amplitude(),
            bar_height,
            peak_bar_height
        );

        let y_coord = frame.height() - bar_height;
        let peak_y_coord = frame.height() - peak_bar_height;

        // Peak Volume
        frame.fill_rectangle(
            Point::new(offset_x, peak_y_coord),
            Size::new(bar_width, peak_bar_height),
            Fill::from(darken_color(Colors::success(), 0.4)),
        );
        // RMS Volume
        frame.fill_rectangle(
            Point::new(offset_x, y_coord),
            Size::new(bar_width, bar_height),
            Fill::from(Colors::success()),
        );
    }

    /// Draw the volume handle
    fn draw_volume_handle(frame: &mut Frame, offset_x: f32, volume: Decibels) {
        let mut handle_path = iced::widget::canvas::path::Builder::new();
        let handle_width = 10.0;

        let start_x = offset_x - handle_width / 2.;
        let tick_y = Self::decibels_y_position(frame, volume).clamp(0.0, frame.height());
        let start_point = Point::new(start_x, tick_y);

        handle_path.move_to(start_point);
        handle_path.line_to(start_point + Vector::new(handle_width, handle_width / 2.0));
        handle_path.line_to(start_point + Vector::new(handle_width, -handle_width / 2.0));
        handle_path.line_to(start_point);

        frame.fill(
            &handle_path.build(),
            Fill::from(darken_color(Colors::border_color(), -0.5)),
        );
    }

    /// Draw a mark and text reference for the `value` decibels point.
    fn draw_mark(frame: &mut Frame, bar_width: f32, offset_x: f32, value: Decibels) {
        let text = format!("{:>4.0}", value.as_db());
        let tick_y = Self::decibels_y_position(frame, value);
        let mut tick_path = iced::widget::canvas::path::Builder::new();
        tick_path.move_to(Point::new(offset_x, tick_y));
        tick_path.line_to(Point::new(offset_x + bar_width, tick_y));
        frame.stroke(
            &tick_path.build(),
            Stroke::default().with_color(Colors::border_color()),
        );
        frame.translate(Vector::new(bar_width * 2. + 5., tick_y - 8.0));
        frame.fill_text(iced::widget::canvas::Text {
            content: text,
            color: Colors::border_color(),
            ..iced::widget::canvas::Text::default()
        });
        frame.translate(Vector::new(-(bar_width * 2. + 5.), -(tick_y - 8.0)));
    }

    fn y_perc_to_db(y_perc: f32) -> Decibels {
        let max_ampl = Self::max_amplitude();
        let min_ampl = Self::min_amplitude();
        let db = interpolate(y_perc, (0.0, 1.0), (min_ampl, max_ampl));
        Decibels::from_db(render_to_db(db))
    }

    /// The Y coordinate for a given `value` in decibels. The return value is reversed.
    ///
    /// This is for rendering values between -Infinity decibels and `VolumeMeterProgram::max_amplitude`
    /// decibels.
    fn decibels_y_position(frame: &mut Frame, value: Decibels) -> f32 {
        let max_ampl = Self::max_amplitude();
        let min_ampl = Self::min_amplitude();
        let value = db_to_render(value.as_db());
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

fn render_to_db(render: f32) -> f32 {
    let reference_amplitude = 1e-1;
    60.0 * (render / reference_amplitude).log10()
}

fn interpolate(value: f32, range_from: (f32, f32), range_to: (f32, f32)) -> f32 {
    let bounds_from = range_from.1 - range_from.0;
    let bounds_to = range_to.1 - range_to.0;
    range_to.0 + (value - range_from.0) / bounds_from * bounds_to
}

#[cfg(feature = "story")]
pub mod story {
    use std::time::{Duration, Instant};

    use iced::{widget::Row, Subscription};

    use audio_processor_iced_design_system::style::Container1;
    use audio_processor_iced_storybook::StoryView;

    use super::*;

    struct Story {
        volume_info: VolumeInfo,
        state: Vec<VolumeMeter>,
        start_time: Instant,
    }

    pub fn default() -> impl StoryView<Message> {
        Story {
            volume_info: VolumeInfo::default(),
            state: (0..20).map(|_| VolumeMeter::new()).collect(),
            start_time: Instant::now(),
        }
    }

    impl StoryView<Message> for Story {
        fn update(&mut self, _message: Message) -> Command<Message> {
            let random_volume = 0.06
                * (0.1
                    + ((1.0 + (0.001 * self.start_time.elapsed().as_millis() as f32).sin()) / 2.0));
            let random_volume = random_volume.into();
            self.volume_info.left = random_volume;
            self.volume_info.left_peak = random_volume * 1.1;
            self.volume_info.right = random_volume;
            self.volume_info.right_peak = random_volume * 1.1;
            for s in &mut self.state {
                s.set_volume_info(self.volume_info);
            }
            Command::none()
        }

        fn subscription(&self) -> Subscription<Message> {
            iced::time::every(Duration::from_millis(16)).map(|_| Message::None)
        }

        fn view(&self) -> Element<Message> {
            let children = self
                .state
                .iter()
                .map(|state| {
                    Container::new(state.view())
                        .style(Container1::default().border())
                        .width(Length::Fixed((Spacing::base_control_size() * 2) as f32))
                        .padding(Spacing::base_spacing())
                        .height(Length::Fill)
                        .into()
                })
                .collect();
            Row::with_children(children).into()
        }
    }
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::assert_f_eq;

    use super::*;

    #[test]
    fn test_interpolate() {
        assert_f_eq!(interpolate(1., (0., 1.), (0., 2.)), 2.);
    }

    #[test]
    fn test_interpolate_negative_range() {
        assert_f_eq!(interpolate(0., (-1., 1.), (0., 2.)), 1.);
    }

    #[test]
    fn test_interpolate_reversed_range() {
        assert_f_eq!(interpolate(0., (-1., 1.), (2., 0.)), 1.);
    }
}
