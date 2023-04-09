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
use audio_processor_testing_helpers::relative_path;
use audio_processor_traits::{AudioBuffer, AudioContext, AudioProcessor};
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{BufferSize, SampleRate, StreamConfig};

fn main() {
    wisual_logger::init_from_env();
    let args: Vec<String> = std::env::args().collect();
    let file_path = args
        .get(1)
        .cloned()
        .unwrap_or(relative_path!("../../../../input-files/bass.wav"));

    let mut processor = audio_processor_file::AudioFileProcessor::from_path(
        audio_garbage_collector::handle(),
        Default::default(),
        &file_path,
    )
    .unwrap();
    let mut context = AudioContext::default();
    processor.prepare(&mut context);
    run_audio(processor);
}

fn run_audio(mut processor: impl AudioProcessor<SampleType = f32> + Send + 'static) {
    let host = cpal::default_host();
    let output_device = host.default_output_device().unwrap();
    let mut context = AudioContext::default();
    let mut buffer = AudioBuffer::empty();
    buffer.resize(2, 1024);
    let _handle = output_device
        .build_output_stream(
            &StreamConfig {
                buffer_size: BufferSize::Default,
                channels: 2,
                sample_rate: SampleRate(44100),
            },
            move |data, _info| {
                buffer.resize(2, data.len() / 2);
                buffer.copy_from_interleaved(data);
                processor.process(&mut context, &mut buffer);
            },
            |err| log::error!("CPAL stream error: {}", err),
        )
        .unwrap();
    std::thread::park();
}
