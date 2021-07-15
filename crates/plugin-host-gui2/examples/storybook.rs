use derive_more::{From, TryInto};
use plugin_host_gui2::ui;

#[derive(Debug, From, Clone, TryInto)]
enum Message {
    AudioIOSettings(ui::audio_io_settings::Message),
    TransportControls(ui::main_content_view::transport_controls::Message),
    None(()),
}

fn main() -> iced::Result {
    audio_processor_iced_storybook::builder::<Message>()
        .story("AudioIOSettings", ui::audio_io_settings::story::default())
        .story(
            "Transport controls",
            ui::main_content_view::transport_controls::story::default(),
        )
        .story(
            "Status bar",
            ui::main_content_view::status_bar::story::default(),
        )
        .run()
}
