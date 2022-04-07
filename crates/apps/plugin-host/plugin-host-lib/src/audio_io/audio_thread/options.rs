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
use cpal::SampleRate;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AudioHostId {
    Default,
    Id(String),
}

impl std::fmt::Display for AudioHostId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioHostId::Default => write!(f, "Default audio host"),
            AudioHostId::Id(str) => write!(f, "{}", str),
        }
    }
}

impl Default for AudioHostId {
    fn default() -> Self {
        AudioHostId::Default
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AudioDeviceId {
    Default,
    Id(String),
}

impl std::fmt::Display for AudioDeviceId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioDeviceId::Default => write!(f, "Default audio device"),
            AudioDeviceId::Id(str) => write!(f, "{}", str),
        }
    }
}

impl Default for AudioDeviceId {
    fn default() -> Self {
        AudioDeviceId::Default
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BufferSize {
    Default,
    Fixed(usize),
}

impl Default for BufferSize {
    fn default() -> Self {
        BufferSize::Fixed(512)
    }
}

impl From<BufferSize> for cpal::BufferSize {
    fn from(value: BufferSize) -> Self {
        match value {
            BufferSize::Default => cpal::BufferSize::Default,
            BufferSize::Fixed(size) => cpal::BufferSize::Fixed(size as u32),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AudioThreadOptions {
    pub host_id: AudioHostId,
    pub output_device_id: AudioDeviceId,
    pub input_device_id: Option<AudioDeviceId>,
    pub buffer_size: BufferSize,
    pub num_channels: usize,
    pub sample_rate: SampleRate,
}

impl Default for AudioThreadOptions {
    fn default() -> Self {
        Self::new(
            Default::default(),
            Default::default(),
            None,
            BufferSize::Fixed(512),
            2,
            SampleRate(44100),
        )
    }
}

impl AudioThreadOptions {
    pub fn new(
        host_id: AudioHostId,
        output_device_id: AudioDeviceId,
        input_device_id: Option<AudioDeviceId>,
        buffer_size: BufferSize,
        num_channels: usize,
        sample_rate: SampleRate,
    ) -> Self {
        AudioThreadOptions {
            host_id,
            output_device_id,
            input_device_id,
            buffer_size,
            num_channels,
            sample_rate,
        }
    }
}
