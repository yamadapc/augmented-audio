# audio-processor-iced-storybook


This is a very simple draft of a "storybook" style library for iced.

It's similar in spirit to ["Storybook for react"](https://storybook.js.org/docs/react/get-started/introduction).

![](https://raw.githubusercontent.com/yamadapc/augmented-audio/master/crates/augmented/gui/audio-processor-iced-storybook/screenshot.png)

As of current it simply:
* Shows a sidebar with story names
* Shows stories in the content view when they're clicked

### Usage
Stories are configured using the `builder` function. They're registered by name in **two** possible ways.

The first way is to declare the story as a `story_fn`, which is just for rendering stateless elements:
```rust
use iced::{Text, Length, Container};

use audio_processor_iced_storybook as storybook;

type Message = ();

fn main() {
    storybook::builder::<Message>()
        .story_fn("Hello world", || {
            Container::new(Text::new("Hey!"))
                .padding(50)
                .center_x()
                .center_y()
                .height(Length::Fill)
                .width(Length::Fill)
                .into()
        })
        .run()
        .unwrap();
}
```
`storybook::builder` takes the children message type as a type-parameter. It's required that this type conforms to
`'static + Clone + Debug`.

Stories may have different `Message` types, as long as they're convertible to the root type via `From`.

The second way of registering stories is as follows:

You've a `my_view` module which declares a button view
```rust
mod my_view {
    use iced::*;

    // This view has the state of the button
    pub struct MyView {
        button_state: iced::button::State,
    }

    // This view fires a `Message::ButtonClicked` message
    #[derive(Clone, Debug)]
    pub enum Message {
        ButtonClicked,
    }

    impl MyView {
        pub fn new() -> Self {
            Self {
                button_state: iced::button::State::default(),
            }
        }

        pub fn view(&mut self) -> Element<Message> {
            Button::new(&mut self.button_state, Text::new("Hello world"))
                .on_press(Message::ButtonClicked)
                .into()
        }
    }

    // You will declare a `story` module, which may be conditionally compiled on your set-up
    pub mod story {
        use audio_processor_iced_storybook::StoryView;

        use super::*;

        // You will declare some helper types
        struct Story(MyView);
        pub fn default() -> Story {
            Story::default()
        }

        // You will implement the `StoryView` trait for your story. This will be parameterized over the `Message` type,
        // however it doesn't have to be a global type, as long as the root type is convertible to/from this.
        impl StoryView<Message> for Story {
            // You may implement an update function for your story
            fn update(&mut self, _message: Message) -> Command<Message> { Command::none() }

            // You will implement the view function
            fn view(&mut self) -> Element<Message> {
                self.0.view()
            }
        }
    }
}

// In order to have different message types, you'll implement a "super-type" for Message, which derives `From` and
// `TryInto`

use derive_more::{From, TryInto}; // <- You need this to derive `From`/`TryInto` automatically for the child message

#[derive(Debug, From, Clone, TryInto)]
enum Message {
    MyView(my_view::Message),
    None(()) // <- Adding a `None(())` will let you continue using stateless stories as well.
}

// examples/stories.rs
fn main() {
    audio_processor_iced_storybook::builder::<Message>()
        // You will register the story with `story` rather than `story_fn`.
        .story("MyView - default", my_view::story::default())
        .run()
        .unwrap();
}
```

For better examples, see [`crates/plugin-host-gui2`](https://github.com/yamadapc/augmented-audio/tree/master/crates/plugin-host-gui2).


License: MIT
