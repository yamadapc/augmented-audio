use iced::Text;

fn main() -> iced::Result {
    audio_processor_iced_storybook::builder::<()>()
        .story_fn("Hello world 1", || Text::new("Hello world 1").into())
        .story_fn("Hello world 2", || Text::new("Hello world 2").into())
        .story_fn("Hello world 3", || Text::new("Hello world 3").into())
        .story_fn("Hello world 4", || Text::new("Hello world 4").into())
        .run()
}
