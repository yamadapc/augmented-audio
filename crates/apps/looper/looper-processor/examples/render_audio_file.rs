use cacao::appkit::window::Window;
use cacao::appkit::{App, AppDelegate};
use cacao::view::View;

use audio_processor_file::AudioFileProcessor;
use audio_processor_traits::{AudioBuffer, AudioProcessor, OwnedAudioBuffer, VecAudioBuffer};

#[allow(unused)]
fn read_test_buffer() -> VecAudioBuffer<f32> {
    let input = audio_processor_testing_helpers::relative_path!("../../../../input-files/bass.mp3");
    let input = std::path::Path::new(&input).canonicalize().unwrap();

    let mut input_file = AudioFileProcessor::from_path(
        audio_garbage_collector::handle(),
        Default::default(),
        input.to_str().unwrap(),
    )
    .unwrap();

    input_file.prepare(Default::default());
    let input_file = input_file.buffer();

    let mut buffer = VecAudioBuffer::new();
    buffer.resize(input_file.len(), input_file[0].len(), 0.0);
    for (c, channel) in input_file.iter().enumerate() {
        for (s, sample) in channel.iter().enumerate() {
            buffer.set(c, s, *sample);
        }
    }
    buffer
}

#[derive(Default)]
struct RenderAudioFileApp {
    window: Window,
    content_view: View,
}

impl AppDelegate for RenderAudioFileApp {
    fn did_finish_launching(&self) {
        self.window.set_minimum_content_size(400., 400.);
        self.window.set_title("Render audio file");
        self.window.show();

        self.content_view
            .set_background_color(cacao::color::Color::SystemPink);
        self.window.set_content_view(&self.content_view)
    }
}

fn main() {
    wisual_logger::init_from_env();
    App::new("com.beijaflor.loadaudio", RenderAudioFileApp::default()).run();
}
