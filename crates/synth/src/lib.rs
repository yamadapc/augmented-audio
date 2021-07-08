use num::FromPrimitive;
use rimd::Status;

use audio_processor_traits::{
    AudioBuffer, AudioProcessor, AudioProcessorSettings, MidiEventHandler, MidiMessageLike,
};
use filter::LowPassFilterProcessor;
use num::traits::FloatConst;
use voice::Voice;

mod filter;
mod voice;

pub struct Synthesizer {
    voices: [Voice; 4],
    filter: LowPassFilterProcessor,
}

impl Synthesizer {
    pub fn new(sample_rate: f32) -> Self {
        Synthesizer {
            voices: [
                Voice::new(sample_rate),
                Voice::new(sample_rate),
                Voice::new(sample_rate),
                Voice::new(sample_rate),
            ],
            filter: LowPassFilterProcessor::new(),
        }
    }
}

impl AudioProcessor for Synthesizer {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        for voice in &mut self.voices {
            voice.prepare(settings);
        }
        self.filter.prepare(settings);
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        // Silence the input
        for sample_index in 0..data.num_samples() {
            for channel_index in 0..data.num_channels() {
                data.set(channel_index, sample_index, 0.0);
            }
        }

        // Produce output for 4 voices
        for voice in &mut self.voices {
            voice.process(data);
        }

        self.filter.process(data);
    }
}

impl MidiEventHandler for Synthesizer {
    fn process_midi_events<Message: MidiMessageLike>(&mut self, midi_messages: &[Message]) {
        for message in midi_messages {
            let maybe_bytes = message.bytes();
            // TODO Write a better MIDI parser
            // (rimd allocates on parse, so we can't parse the whole message; only the status byte)
            let maybe_status = maybe_bytes.map(|b| rimd::Status::from_u8(b[0])).flatten();
            if let Some((status, bytes)) = maybe_status.zip(maybe_bytes) {
                self.handle_midi_message(status, bytes);
            }
        }
    }
}

impl Synthesizer {
    fn handle_midi_message(&mut self, status: rimd::Status, bytes: &[u8]) {
        match status {
            Status::NoteOn => {
                self.note(bytes);
            }
            Status::ControlChange => {
                if bytes[1] == 21 {
                    self.filter.set_cutoff(22000.0 * (bytes[2] as f32 / 127.0));
                }
                if bytes[1] == 22 {
                    self.filter.set_q(1.0 + (bytes[2] as f32 / 127.0));
                }
            }
            _ => {}
        }
    }

    fn note(&mut self, bytes: &[u8]) {
        let note = bytes[1];
        let velocity = bytes[2];
        if velocity == 0 {
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
                self.voices[0].note_on(note, velocity);
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_compiles() {}
}
