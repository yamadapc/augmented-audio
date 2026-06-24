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
use augmented_midi::{CONTROL_CHANGE_MASK, NOTE_OFF_MASK, NOTE_ON_MASK};

use audio_processor_traits::simple_processor::MultiChannel;
use audio_processor_traits::{
    AudioBuffer, AudioContext, AudioProcessor, AudioProcessorSettings, MidiEventHandler,
    MidiMessageLike,
};
use augmented_dsp_filters::rbj::{FilterProcessor, FilterType};
use voice::Voice;

mod voice;

pub struct Synthesizer {
    current_voice: usize,
    voices: [Voice; 4],
    filter: MultiChannel<FilterProcessor<f32>>,
}

impl Default for Synthesizer {
    fn default() -> Self {
        let settings = AudioProcessorSettings::default();
        Self::new(settings.sample_rate)
    }
}

impl Synthesizer {
    pub fn new(sample_rate: f32) -> Self {
        Synthesizer {
            current_voice: 0,
            voices: [
                Voice::new(sample_rate),
                Voice::new(sample_rate),
                Voice::new(sample_rate),
                Voice::new(sample_rate),
            ],
            filter: MultiChannel::new(|| FilterProcessor::new(FilterType::LowPass)),
        }
    }
}

impl AudioProcessor for Synthesizer {
    type SampleType = f32;

    fn prepare(&mut self, context: &mut AudioContext) {
        for voice in &mut self.voices {
            voice.prepare(context);
        }
        self.filter.prepare(context);
    }

    fn process(&mut self, context: &mut AudioContext, data: &mut AudioBuffer<Self::SampleType>) {
        // Silence the input
        for sample in data.slice_mut() {
            *sample = 0.0;
        }

        // Produce output for 4 voices
        for voice in &mut self.voices {
            voice.process(context, data);
        }

        self.filter.process(context, data);
    }
}

impl MidiEventHandler for Synthesizer {
    fn process_midi_events<Message: MidiMessageLike>(&mut self, midi_messages: &[Message]) {
        for message in midi_messages {
            let maybe_bytes = message.bytes();
            if let Some(bytes) = maybe_bytes {
                self.handle_midi_message(bytes);
            }
        }
    }
}

impl Synthesizer {
    fn handle_midi_message(&mut self, bytes: &[u8]) {
        match bytes[0] & 0xF0 {
            NOTE_ON_MASK => {
                self.note(false, bytes);
            }
            NOTE_OFF_MASK => {
                // println!("Note off {}", bytes[1]);
                self.note(true, bytes);
            }
            CONTROL_CHANGE_MASK => {
                if bytes[1] == 21 {
                    self.filter.for_each(|filter| {
                        filter.set_cutoff(22000.0 * (bytes[2] as f32 / 127.0));
                    })
                }
                if bytes[1] == 22 {
                    self.filter.for_each(|filter| {
                        filter.set_cutoff(22000.0 * (bytes[2] as f32 / 127.0));
                    })
                }
            }
            _ => {}
        }
    }

    fn note(&mut self, is_off: bool, bytes: &[u8]) {
        let note = bytes[1];
        let velocity = bytes[2];
        if velocity == 0 || is_off {
            let voice = self.voices.iter_mut().find(|voice| {
                voice.current_note().is_some() && voice.current_note().unwrap() == note
            });
            if let Some(voice) = voice {
                voice.note_off();
            }
        } else {
            let voice = self
                .voices
                .iter_mut()
                .find(|voice| voice.current_note().is_none());
            if let Some(voice) = voice {
                voice.note_on(note, velocity);
            } else {
                self.current_voice = (self.current_voice + 1) % self.voices.len();
                self.voices[self.current_voice].note_on(note, velocity);
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_compiles() {}
}
