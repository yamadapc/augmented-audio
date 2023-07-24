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

//! Input handling should push samples onto a ring-buffer.

use std::sync::mpsc::Sender;

use cpal::{traits::DeviceTrait, StreamConfig};
use ringbuf::Producer;

use crate::standalone_cpal::error::AudioThreadError;

pub fn build_input_stream<Device: DeviceTrait>(
    input_device: Device,
    input_config: StreamConfig,
    mut producer: Producer<f32>,
    errors_tx: Sender<AudioThreadError>,
) -> Result<Device::Stream, AudioThreadError> {
    let input_stream = input_device
        .build_input_stream(
            &input_config,
            move |data: &[f32], _input_info: &cpal::InputCallbackInfo| {
                input_stream_callback(&mut producer, data)
            },
            move |err| {
                log::error!("Input error: {:?}", err);
                let _ = errors_tx.send(AudioThreadError::InputStreamError(err));
            },
            None,
        )
        .map_err(AudioThreadError::BuildInputStreamError)?;
    Ok(input_stream)
}

fn input_stream_callback(producer: &mut Producer<f32>, data: &[f32]) {
    for sample in data {
        while producer.push(*sample).is_err() {}
    }
}
