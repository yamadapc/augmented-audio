use iced::{Element, Text};

use plugin_host_lib::audio_io::StartError;

pub struct StartErrorView;

impl StartErrorView {
    pub fn view(error: &StartError) -> Element<()> {
        Text::new(format!("Failed starting host: {:?}", error)).into()
    }
}
