use derive_more::{From, TryInto};
use plugin_host_gui2::ui;
use ui::audio_io_settings;
use ui::audio_io_settings::dropdown_with_label;
use ui::main_content_view::{plugin_content, status_bar, transport_controls, volume_meter};

#[derive(Debug, From, Clone, TryInto)]
enum Message {
    AudioIOSettings(audio_io_settings::view::Message),
    TransportControls(transport_controls::Message),
    Dropdown(String),
    PluginContent(plugin_content::Message),
    VolumeMeter(volume_meter::Message),
    None(()),
}

fn main() -> iced::Result {
    audio_processor_iced_storybook::builder::<Message>()
        .story("Dropdown with label", dropdown_with_label::story::default())
        .story("AudioIOSettings", audio_io_settings::view::story::default())
        .story("Transport controls", transport_controls::story::default())
        .story("Plugin content", plugin_content::story::default())
        .story("Volume meter", volume_meter::story::default())
        .story("Status bar", status_bar::story::default())
        .run()
}
