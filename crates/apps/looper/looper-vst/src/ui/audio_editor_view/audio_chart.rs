use iced::canvas::Fill;
use iced::canvas::Frame;
use iced::canvas::Geometry;
use iced::canvas::Stroke;

pub fn draw_samples_chart(
    frame: &mut Frame,
    width: f32,
    height: f32,
    offset: f32,
    num_samples: f32,
    samples: impl Iterator<Item = f32>,
) {
    let color = Colors::active_border_color();
    let num_pixels = (width * 8.0).max(1000.0);
    let step_size = ((num_samples / num_pixels) as usize).max(1);

    let samples = samples.enumerate().step_by(step_size);
    let points = samples
        .map(|(index, item)| {
            let f_index = index as f32;
            let x = (f_index / num_samples) * width;
            let y = item * height / 2.0 + frame.height() / 2.0;
            (x, y)
        })
        .filter(|(x, _)| *x >= offset && x.is_finite())
        .take_while(|(x, _)| *x < frame.width() + offset);

    let mut path = iced::canvas::path::Builder::new();
    points.for_each(|(x, y)| path.line_to(Point::new(x, y)));
    frame.stroke(&path.build(), Stroke::default().with_color(color));
}

pub fn draw_rms_chart<'a>(
    frame: &mut Frame,
    // Total width the audio file is projected to occupy
    total_width: f32,
    // Total height the audio file is projected to occupy
    total_height: f32,
    offset: f32,
    num_samples: f32,
    samples: impl Iterator<Item = f32>,
) {
    let color = Colors::active_border_color();
    let num_pixels = (total_width * 2.0).max(1000.0);
    let step_size = ((num_samples / num_pixels) as usize).max(1);

    let h = frame.height();
    let w = frame.width();

    let samples = samples.enumerate().step_by(step_size);
    let points: Vec<(f32, f32)> = samples
        .map(|(index, item)| {
            let f_index = index as f32;
            let x = (f_index / num_samples) * total_width;
            let y = item * total_height + h / 2.0;
            (x, y)
        })
        .filter(|(x, _)| *x >= offset && x.is_finite())
        .take_while(|(x, _)| *x < w + offset)
        .collect();

    let start_path = || {
        let mut path = iced::canvas::path::Builder::new();
        path.line_to(Point::new(0.0, h / 2.0));
        path
    };
    let end_path = |path: &mut iced::canvas::path::Builder| {
        path.line_to(Point::new(w, h / 2.0));
        path.line_to(Point::new(0.0, h / 2.0));
    };

    // Draw top line
    let mut path = start_path();
    points
        .iter()
        .cloned()
        .for_each(|(x, y)| path.line_to(Point::new(x, y)));
    end_path(&mut path);
    frame.fill(&path.build(), Fill::from(color));

    // Draw bottom line
    let points = points
        .iter()
        .cloned()
        .map(|(x, y)| (x, (y - h / 2.0) * -1.0 + h / 2.0));
    let mut path = start_path();
    points.for_each(|(x, y)| path.line_to(Point::new(x, y)));
    end_path(&mut path);
    frame.fill(&path.build(), Fill::from(color));
}
