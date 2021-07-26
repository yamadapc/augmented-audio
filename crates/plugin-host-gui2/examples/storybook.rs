use derive_more::{From, TryInto};
use plugin_host_gui2::ui;

#[derive(Debug, From, Clone, TryInto)]
enum Message {
    AudioIOSettings(ui::audio_io_settings::Message),
    TransportControls(ui::main_content_view::transport_controls::Message),
    Dropdown(String),
    PluginContent(ui::main_content_view::plugin_content::view::Message),
    None(()),
}

fn main() -> iced::Result {
    audio_processor_iced_storybook::builder::<Message>()
        .story(
            "Dropdown with label",
            ui::audio_io_settings::dropdown_with_label::story::default(),
        )
        .story(
            "AudioIOSettings",
            ui::audio_io_settings::view::story::default(),
        )
        .story(
            "Transport controls",
            ui::main_content_view::transport_controls::story::default(),
        )
        .story(
            "Plugin content",
            ui::main_content_view::plugin_content::view::story::default(),
        )
        .story(
            "Status bar",
            ui::main_content_view::status_bar::story::default(),
        )
        .run()
}
