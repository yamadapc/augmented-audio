use derive_more::{From, TryInto};

use augmented::ops::wisual_logger;

#[derive(Debug, From, Clone, TryInto)]
enum Message {
    AudioEditorView(loopi::ui::audio_editor_view::Message),
    None(()),
}

fn main() -> iced::Result {
    wisual_logger::init_from_env();

    audio_processor_iced_storybook::builder::<Message>()
        .story(
            "Audio editor",
            loopi::ui::audio_editor_view::story::default(),
        )
        .run()
}
