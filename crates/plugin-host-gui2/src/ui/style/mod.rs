use iced::{Background, Color};

pub struct ContainerStylesheet {
    pub text_color: Option<Color>,
    pub background: Option<Background>,
    pub border_radius: f32,
    pub border_width: f32,
    pub border_color: Color,
}

impl ContainerStylesheet {
    #[allow(dead_code)]
    pub fn with_text_color(mut self, color: Option<Color>) -> Self {
        self.text_color = color;
        self
    }

    #[allow(dead_code)]
    pub fn with_background(mut self, background: Option<Background>) -> Self {
        self.background = background;
        self
    }

    #[allow(dead_code)]
    pub fn with_border_radius(mut self, border_radius: f32) -> Self {
        self.border_radius = border_radius;
        self
    }

    #[allow(dead_code)]
    pub fn with_border_width(mut self, border_width: f32) -> Self {
        self.border_width = border_width;
        self
    }

    #[allow(dead_code)]
    pub fn with_border_color(mut self, border_color: Color) -> Self {
        self.border_color = border_color;
        self
    }
}

impl Default for ContainerStylesheet {
    fn default() -> Self {
        iced::container::Style::default().into()
    }
}

impl From<iced::container::Style> for ContainerStylesheet {
    fn from(style: iced::container::Style) -> Self {
        Self {
            text_color: style.text_color,
            background: style.background,
            border_radius: style.border_radius,
            border_width: style.border_width,
            border_color: style.border_color,
        }
    }
}

impl iced::container::StyleSheet for ContainerStylesheet {
    fn style(&self) -> iced::container::Style {
        iced::container::Style {
            text_color: self.text_color.clone(),
            background: self.background.clone(),
            border_radius: self.border_radius,
            border_width: self.border_width,
            border_color: self.border_color,
        }
    }
}
