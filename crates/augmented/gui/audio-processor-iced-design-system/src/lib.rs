pub mod charts;
pub mod colors;
pub mod container;
pub mod dropdown_with_label;
pub mod knob;
pub mod menu_list;
pub mod router;
pub mod spacing;
pub mod style;
pub mod tabs;
pub mod tree_view;
pub mod updatable;

pub fn default_settings<Flags: Default>() -> iced::Settings<Flags> {
    iced::Settings {
        antialiasing: true,
        default_text_size: spacing::Spacing::default_font_size(),
        window: iced::window::Settings {
            size: (1400, 1024),
            ..iced::window::Settings::default()
        },
        ..iced::Settings::default()
    }
}
