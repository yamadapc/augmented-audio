use derive_more::{From, TryInto};

#[derive(Debug, From, Clone, TryInto)]
enum Message {
    Dropdown(String),
    Settings(audio_settings_gui::Message),
    None(()),
}

fn main() -> iced::Result {
    audio_processor_iced_storybook::builder::<Message>()
        .story(
            "Dropdown with label",
            audio_settings_gui::dropdown_with_label::story::default(),
        )
        .story("AudioIOSettings", audio_settings_gui::story::default())
        .run()
}
