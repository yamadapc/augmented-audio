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

//! CPAL output stream handling. This reads input samples (if there's an input
//! stream) and ticks the audio processor forward.
//!
//! It will also forward MIDI events that happened between frames.

use audio_processor_traits::{AudioBuffer, AudioProcessor, InterleavedAudioBuffer};
use cpal::{traits::DeviceTrait, StreamConfig};
use ringbuf::Consumer;

use crate::StandaloneProcessor;

use super::error::AudioThreadError;
use super::midi::MidiContext;

/// Build the output callback stream with CPAL and return it.
pub fn build_output_stream<Device: DeviceTrait>(
    mut app: impl StandaloneProcessor,
    mut midi_context: Option<MidiContext>,
    num_output_channels: usize,
    num_input_channels: usize,
    mut input_consumer: Consumer<f32>,
    output_device: Device,
    output_config: StreamConfig,
) -> Result<Device::Stream, AudioThreadError> {
    // Output callback section
    log::info!(
        "num_input_channels={} num_output_channels={} sample_rate={}",
        num_input_channels,
        num_output_channels,
        output_config.sample_rate.0
    );
    let output_stream = output_device
        .build_output_stream(
            &output_config,
            move |data: &mut [f32], _output_info: &cpal::OutputCallbackInfo| {
                output_stream_with_context(OutputStreamFrameContext {
                    midi_context: midi_context.as_mut(),
                    processor: &mut app,
                    num_input_channels,
                    num_output_channels,
                    consumer: &mut input_consumer,
                    data,
                });
            },
            |err| {
                log::error!("Playback error: {:?}", err);
            },
        )
        .map_err(AudioThreadError::BuildOutputStreamError)?;

    Ok(output_stream)
}

/// Data borrowed to process a single output frame.
struct OutputStreamFrameContext<'a, SP: StandaloneProcessor> {
    midi_context: Option<&'a mut MidiContext>,
    processor: &'a mut SP,
    num_input_channels: usize,
    num_output_channels: usize,
    consumer: &'a mut Consumer<f32>,
    data: &'a mut [f32],
}

/// Tick one frame of the output stream.
///
/// This will be called repeatedly for every audio buffer we must produce.
fn output_stream_with_context<SP: StandaloneProcessor>(context: OutputStreamFrameContext<SP>) {
    let OutputStreamFrameContext {
        midi_context,
        processor,
        num_input_channels,
        num_output_channels,
        consumer,
        data,
    } = context;
    let mut audio_buffer = InterleavedAudioBuffer::new(num_output_channels, data);

    for frame in audio_buffer.frames_mut() {
        if num_input_channels == num_output_channels {
            for sample in frame {
                if let Some(input_sample) = consumer.pop() {
                    *sample = input_sample;
                } else {
                }
            }
        } else if let Some(input_sample) = consumer.pop() {
            // This only works if num_input_channels == 1
            for sample in frame {
                *sample = input_sample
            }
        } else {
            break;
        }
    }

    // Collect MIDI
    super::midi::flush_midi_events(midi_context, processor);

    processor.processor().process(&mut audio_buffer);
}

#[cfg(test)]
mod test {
    use audio_processor_traits::{AudioBuffer, AudioProcessor};

    use crate::{StandaloneAudioOnlyProcessor, StandaloneProcessor};

    use super::*;

    #[test]
    fn test_tick_output_stream_reads_from_consumer_and_calls_process() {
        struct MockProcessor {
            inputs: Vec<f32>,
        }
        impl AudioProcessor for MockProcessor {
            type SampleType = f32;

            fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
                &mut self,
                data: &mut BufferType,
            ) {
                for i in data.slice_mut() {
                    self.inputs.push(*i);
                    *i = *i * 2.0;
                }
            }
        }

        let buf = ringbuf::RingBuffer::new(10);
        let (mut producer, mut consumer) = buf.split();
        let processor = MockProcessor { inputs: vec![] };
        let mut processor: StandaloneAudioOnlyProcessor<MockProcessor> =
            StandaloneAudioOnlyProcessor::new(processor, Default::default());

        for i in 0..10 {
            producer.push(i as f32).expect("Pushing sample failed");
        }

        let mut data = [0.0; 10];
        let context = OutputStreamFrameContext {
            processor: &mut processor,
            consumer: &mut consumer,
            num_output_channels: 1,
            num_input_channels: 1,
            midi_context: None,
            data: &mut data,
        };
        output_stream_with_context(context);

        assert_eq!(
            processor.processor().inputs,
            vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
        );
        assert_eq!(
            data,
            [0.0, 2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0, 18.0]
        )
    }
}
