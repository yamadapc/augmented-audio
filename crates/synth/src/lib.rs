use adsr_envelope::Envelope;
use audio_processor_traits::{
    AudioBuffer, AudioProcessor, AudioProcessorSettings, MidiEventHandler, MidiMessageLike,
};
use num::FromPrimitive;
use oscillator::Oscillator;
use rimd::Status;

struct Voice {
    oscillator: Oscillator<f32>,
    envelope: Envelope,
    current_note: Option<u8>,
    volume: f32,
}

impl Voice {
    fn new(sample_rate: f32) -> Self {
        Voice {
            oscillator: Oscillator::new_with_sample_rate(
                sample_rate,
                oscillator::generators::square_generator,
            ),
            envelope: Envelope::new(),
            current_note: None,
            volume: 0.25,
        }
    }

    fn note_on(&mut self, note: u8, _velocity: u8) {
        self.current_note = Some(note);
        self.oscillator
            .set_frequency(pitch_calc::hz_from_step(note as f32));
        self.envelope.note_on();
    }

    fn note_off(&mut self) {
        self.current_note = None;
        self.envelope.note_off();
    }
}

impl AudioProcessor for Voice {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.oscillator.set_sample_rate(settings.sample_rate());
        self.envelope.set_sample_rate(settings.sample_rate());
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        for sample_index in 0..data.num_samples() {
            let oscillator_value = self.oscillator.get();
            let envelope_volume = self.envelope.volume();
            let output = self.volume * oscillator_value * envelope_volume;

            for channel_index in 0..data.num_channels() {
                let input = *data.get(channel_index, sample_index);
                data.set(channel_index, sample_index, input + output);
            }

            self.envelope.tick();
            self.oscillator.tick();
        }
    }
}

pub struct Synthesizer {
    voices: [Voice; 4],
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
        }
    }
}

impl AudioProcessor for Synthesizer {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        for voice in &mut self.voices {
            voice.prepare(settings);
        }
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
            _ => {}
        }
    }

    fn note(&mut self, bytes: &[u8]) {
        let note = bytes[1];
        let velocity = bytes[2];
        log::info!("Received NOTE ON message {:?}", bytes);
        if velocity == 0 {
            let voice = self
                .voices
                .iter_mut()
                .find(|voice| voice.current_note.is_some() && voice.current_note.unwrap() == note);
            if let Some(voice) = voice {
                voice.note_off();
            }
        } else {
            let voice = self
                .voices
                .iter_mut()
                .find(|voice| voice.current_note.is_none());
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
