pub mod charts;
pub mod colors;
pub mod container;
pub mod knob;
pub mod menu_list;
pub mod router;
pub mod spacing;
pub mod style;
pub mod tree_view;
pub mod updatable;

pub fn default_settings<Flags: Default>() -> iced::Settings<Flags> {
    iced::Settings {
        antialiasing: true,
        default_text_size: spacing::Spacing::default_font_size(),
        ..iced::Settings::default()
    }
}
