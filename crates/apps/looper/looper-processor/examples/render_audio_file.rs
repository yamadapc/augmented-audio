// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

#[cfg(target_os = "macos")]
mod implementation {
    use cacao::appkit::window::Window;
    use cacao::appkit::{App, AppDelegate};
    use cacao::view::View;

    use audio_processor_file::AudioFileProcessor;
    use audio_processor_traits::{AudioBuffer, AudioContext, AudioProcessor};

    #[allow(unused)]
    fn read_test_buffer() -> AudioBuffer<f32> {
        let input =
            audio_processor_testing_helpers::relative_path!("../../../../input-files/bass.mp3");
        let input = std::path::Path::new(&input).canonicalize().unwrap();

        let mut input_file = AudioFileProcessor::from_path(
            audio_garbage_collector::handle(),
            Default::default(),
            input.to_str().unwrap(),
        )
        .unwrap();

        let mut context = AudioContext::default();
        input_file.prepare(&mut context);
        let input_file = input_file.buffer();

        let mut buffer = AudioBuffer::empty();
        buffer.resize(input_file.len(), input_file[0].len());
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

    pub fn main() {
        wisual_logger::init_from_env();
        App::new("com.beijaflor.loadaudio", RenderAudioFileApp::default()).run();
    }
}

#[cfg(target_os = "macos")]
fn main() {
    implementation::main();
}

#[cfg(not(target_os = "macos"))]
fn main() {
    todo!()
}
