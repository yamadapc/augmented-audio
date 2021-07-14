use iced::Color;

fn dark_blue() -> Color {
    rgb(35, 136, 201)
}

fn white() -> Color {
    rgb(255, 255, 255)
}

fn black() -> Color {
    rgb(19, 19, 19)
}

fn medium_gray() -> Color {
    rgb(42, 42, 42)
}

fn gray() -> Color {
    rgb(50, 50, 50)
}

pub fn light_gray() -> Color {
    rgb(60, 60, 60)
}

fn super_light_gray() -> Color {
    rgb(118, 118, 118)
}

pub fn green() -> Color {
    rgb(73, 190, 84)
}

pub fn red() -> Color {
    rgb(199, 84, 80)
}

pub fn yellow() -> Color {
    rgb(240, 187, 104)
}

pub struct Colors;

impl Colors {
    pub fn text() -> Color {
        white()
    }

    pub fn background_level0() -> Color {
        black()
    }

    pub fn hover_opacity(color: Color) -> Color {
        Color::new(color.r, color.g, color.b, color.a * 0.5)
    }

    pub fn pressed_opacity(color: Color) -> Color {
        Color::new(color.r, color.g, color.b, color.a * 0.4)
    }

    pub fn background_level1() -> Color {
        medium_gray()
    }

    pub fn background_level2() -> Color {
        gray()
    }

    pub fn border_color() -> Color {
        super_light_gray()
    }

    pub fn selected_background() -> Color {
        dark_blue()
    }

    pub fn active_border_color() -> Color {
        dark_blue()
    }
}

fn rgb(r: i32, g: i32, b: i32) -> Color {
    Color::new(r as f32 / 255., g as f32 / 255., b as f32 / 255., 1.)
}
