use std::time::Duration;
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
use enum_dispatch::enum_dispatch;

use audio_processor_file::AudioFileProcessor;
use audio_processor_traits::AudioProcessorSettings;
use augmented_adsr_envelope::Envelope;
use augmented_oscillator::Oscillator;

#[enum_dispatch(MetronomeSoundType)]
pub trait MetronomeSound {
    fn prepare(&mut self, settings: AudioProcessorSettings);
    fn set_accent_beat(&mut self, is_accent: bool);
    fn trigger(&mut self);
    fn process(&mut self) -> f32;
}

#[enum_dispatch]
pub enum MetronomeSoundType {
    Sine(MetronomeSoundSine),
    File(MetronomeSoundFile),
}

impl MetronomeSoundType {
    pub fn file(file: AudioFileProcessor) -> Self {
        Self::File(MetronomeSoundFile {
            file_processor: file,
            envelope: crate::build_envelope(),
        })
    }
}

pub struct MetronomeSoundSine {
    oscillator: Oscillator<f32>,
    envelope: Envelope,
}

impl MetronomeSoundSine {
    pub fn new(sample_rate: f32, envelope: Envelope) -> Self {
        Self {
            oscillator: Oscillator::sine(sample_rate),
            envelope,
        }
    }
}

impl MetronomeSound for MetronomeSoundSine {
    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.envelope.set_sample_rate(settings.sample_rate());
        self.oscillator.set_sample_rate(settings.sample_rate())
    }

    fn set_accent_beat(&mut self, is_accent: bool) {
        if is_accent {
            self.oscillator.set_frequency(880.0);
        } else {
            self.oscillator.set_frequency(440.0);
        }
    }

    fn trigger(&mut self) {
        self.envelope.note_on();
    }

    fn process(&mut self) -> f32 {
        let out = self.envelope.volume() * self.oscillator.get();
        self.envelope.tick();
        self.oscillator.tick();
        out
    }
}

pub struct MetronomeSoundFile {
    file_processor: AudioFileProcessor,
    envelope: Envelope,
}

impl MetronomeSound for MetronomeSoundFile {
    fn prepare(&mut self, settings: AudioProcessorSettings) {
        audio_processor_traits::AudioProcessor::prepare(&mut self.file_processor, settings);
        self.envelope.set_sample_rate(settings.sample_rate());
        // Files have a different envelope to develop the sample a bit more
        self.envelope.set_decay(Duration::from_millis(50));
        self.file_processor.handle().stop();
        self.file_processor.handle().set_should_loop(false);
    }

    fn set_accent_beat(&mut self, _is_accent: bool) {}

    fn trigger(&mut self) {
        self.envelope.note_on();
        self.file_processor.handle().stop();
        self.file_processor.handle().play();
    }

    fn process(&mut self) -> f32 {
        let out = self.envelope.volume() * self.file_processor.process_single().sum::<f32>() / 2.0;
        self.envelope.tick();
        out
    }
}
