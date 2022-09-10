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
use audio_processor_traits::{AudioBuffer, AudioProcessor, AudioProcessorSettings};
use ringbuf::Producer;
use std::sync::mpsc::{channel, Sender};
use symphonia::core::audio::Signal;

struct StreamAudioFileProcessor {
    receiver: ringbuf::Consumer<(f32, f32)>,
}

impl AudioProcessor for StreamAudioFileProcessor {
    type SampleType = f32;

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        for frame in data.frames_mut() {
            if let Some(input_frame) = self.receiver.pop() {
                frame[0] = input_frame.0;
                frame[1] = input_frame.1;
                // println!("{} {}", input_frame.0, input_frame.1);
                // frame[0] = 0.0;
                // frame[1] = 0.0;
            } else {
                frame[0] = 0.0;
                frame[1] = 0.0;
                log::error!("input underrun");
            }
        }
    }
}

fn main() {
    wisual_logger::init_from_env();

    let (mut tx, mut rx) = ringbuf::RingBuffer::new(48000 * 3).split();

    tx.push((0.0, 0.0));
    tx.push((1.0, 0.0));
    tx.push((2.0, 0.0));
    println!("{:?}", rx.pop());
    println!("{:?}", rx.pop());
    println!("{:?}", rx.pop());
    assert!(rx.is_empty());

    let (ready_tx, ready_rx) = channel();
    let _ = std::thread::spawn(move || run_source_thread(tx, ready_tx));
    ready_rx.recv().unwrap();

    let processor = StreamAudioFileProcessor { receiver: rx };
    audio_processor_standalone::audio_processor_main(processor);
}

fn run_source_thread(mut tx: Producer<(f32, f32)>, ready_tx: Sender<()>) {
    log::info!("Buffering");
    let mut input_file = audio_processor_file::file_io::default_read_audio_file(&relative_path!(
        "../../../input-files/bass.mp3"
    ))
    .unwrap();

    let audio_file_stream = input_file.format.default_track().unwrap();
    let mut decoder = symphonia::default::get_codecs()
        .make(&audio_file_stream.codec_params, &Default::default())
        .unwrap();
    let audio_file_stream_id = audio_file_stream.id;

    let mut channel_buffers: Vec<symphonia::core::audio::AudioBuffer<f32>> = Vec::new();
    let mut has_signaled_ready = false;

    let mut samples_read = 0;
    while let Some(packet) = input_file.format.next_packet().ok() {
        if let Some(audio_buffer) = decoder.decode(&packet).ok() {
            let destination =
                audio_processor_file::file_io::convert_audio_buffer_sample_type(audio_buffer);
            let converted_chan_0 = audio_processor_file::file_io::convert_audio_file_sample_rate(
                &destination,
                44100.0,
                0,
            );
            let converted_chan_1 = audio_processor_file::file_io::convert_audio_file_sample_rate(
                &destination,
                44100.0,
                1,
            );
            // println!(
            //     "buffer sample_rate: {} size={}",
            //     destination.spec().rate,
            //     destination.frames()
            // );
            let mut push_item = |el: (f32, f32)| {
                let mut spin_count = 0;
                while let Err(el) = tx.push(el) {
                    if !has_signaled_ready {
                        ready_tx.send(()).unwrap();
                        has_signaled_ready = true;
                        log::info!("Done buffering");
                    } else {
                        // log::error!("output underrun");
                        spin_count += 1;
                    }
                }
            };

            samples_read += destination.frames();

            for sample_index in 0..converted_chan_0.len() {
                push_item((
                    converted_chan_0[sample_index],
                    converted_chan_1[sample_index],
                ));
            }
        } else {
            break;
        }
        // println!("Samples {}", samples_read);
    }

    log::info!("Audio file is finished");
    if !has_signaled_ready {
        ready_tx.send(()).unwrap();
        has_signaled_ready = true;
        log::info!("Done buffering");
    }
}
