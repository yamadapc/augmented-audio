use iced::Color;

fn dark_blue() -> Color {
    rgb(35, 136, 201)
}

fn blue() -> Color {
    rgb(0, 255, 255)
}

fn white() -> Color {
    rgb(255, 255, 255)
}

fn black() -> Color {
    rgb(19, 19, 19)
}

fn medium_gray() -> Color {
    rgb(35, 35, 35)
}

fn gray() -> Color {
    rgb(42, 42, 42)
}

pub fn light_gray() -> Color {
    rgb(49, 49, 49)
}

fn super_light_gray() -> Color {
    rgb(118, 118, 118)
}

pub struct Colors;

impl Colors {
    pub fn text() -> Color {
        white()
    }

    pub fn background_level0() -> Color {
        black()
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
        blue()
    }
}

fn rgb(r: i32, g: i32, b: i32) -> Color {
    Color::new(r as f32 / 255., g as f32 / 255., b as f32 / 255., 1.)
}
