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

/// Options provided to the audio-processor before calling `process`.
#[derive(Clone, PartialEq, Debug, Copy)]
pub struct AudioProcessorSettings {
    /// The sample rate of the signal
    pub sample_rate: f32,
    /// The number of input channels to the signal
    pub input_channels: usize,
    /// The number of output channels to the signal
    pub output_channels: usize,
    /// Buffer size of this processing loop
    pub block_size: usize,
}

impl Default for AudioProcessorSettings {
    fn default() -> Self {
        Self::new(44100.0, 2, 2, 512)
    }
}

impl AudioProcessorSettings {
    /// Create audio processor settings with the given options
    pub fn new(
        sample_rate: f32,
        input_channels: usize,
        output_channels: usize,
        block_size: usize,
    ) -> Self {
        AudioProcessorSettings {
            sample_rate,
            input_channels,
            output_channels,
            block_size,
        }
    }

    /// The sample rate in samples/second as a floating point number
    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    /// The number of input channels
    pub fn input_channels(&self) -> usize {
        self.input_channels
    }

    /// The number of output channels
    pub fn output_channels(&self) -> usize {
        self.output_channels
    }

    /// The number of samples which will be provided on each `process` call
    pub fn block_size(&self) -> usize {
        self.block_size
    }

    /// Set the sample rate of this settings object
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    /// Set the number of input channels of this settings object
    pub fn set_input_channels(&mut self, input_channels: usize) {
        self.input_channels = input_channels;
    }

    /// Set the number of output channels of this settings object
    pub fn set_output_channels(&mut self, output_channels: usize) {
        self.output_channels = output_channels;
    }

    /// Set the buffer size of this settings object
    pub fn set_block_size(&mut self, block_size: usize) {
        self.block_size = block_size;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_settings() {
        let settings = AudioProcessorSettings::default();
        assert_eq!(settings.sample_rate(), 44100.0);
    }

    #[test]
    fn test_new() {
        let settings = AudioProcessorSettings::new(22050.0, 1, 2, 256);
        assert_eq!(settings.sample_rate(), 22050.0);
        assert_eq!(settings.input_channels(), 1);
        assert_eq!(settings.output_channels(), 2);
        assert_eq!(settings.block_size(), 256);
    }

    #[test]
    fn test_set_sample_rate() {
        let mut settings = AudioProcessorSettings::new(22050.0, 1, 2, 256);
        settings.set_sample_rate(44100.0);
        assert_eq!(settings.sample_rate(), 44100.0);
    }

    #[test]
    fn test_set_input_channels() {
        let mut settings = AudioProcessorSettings::new(22050.0, 1, 2, 256);
        settings.set_input_channels(10);
        assert_eq!(settings.input_channels(), 10);
    }

    #[test]
    fn test_set_output_channels() {
        let mut settings = AudioProcessorSettings::new(22050.0, 1, 2, 256);
        settings.set_output_channels(10);
        assert_eq!(settings.output_channels(), 10);
    }

    #[test]
    fn test_set_block_size() {
        let mut settings = AudioProcessorSettings::new(22050.0, 1, 2, 256);
        settings.set_block_size(10);
        assert_eq!(settings.block_size(), 10);
    }
}
