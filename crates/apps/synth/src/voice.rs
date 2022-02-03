use audio_processor_traits::{AudioBuffer, AudioProcessor, AudioProcessorSettings};
use augmented_adsr_envelope::Envelope;
use augmented_oscillator::Oscillator;

pub struct Voice {
    oscillators: [Oscillator<f32>; 3],
    envelope: Envelope,
    current_note: Option<u8>,
    volume: f32,
}

impl Voice {
    pub fn new(sample_rate: f32) -> Self {
        Voice {
            oscillators: [
                Oscillator::new_with_sample_rate(
                    sample_rate,
                    augmented_oscillator::generators::square_generator,
                ),
                Oscillator::new_with_sample_rate(
                    sample_rate,
                    augmented_oscillator::generators::square_generator,
                ),
                Oscillator::new_with_sample_rate(
                    sample_rate,
                    augmented_oscillator::generators::square_generator,
                ),
            ],
            envelope: Envelope::new(),
            current_note: None,
            volume: 0.25,
        }
    }

    pub fn current_note(&self) -> &Option<u8> {
        &self.current_note
    }

    pub fn note_on(&mut self, note: u8, _velocity: u8) {
        self.current_note = Some(note);
        self.oscillators[0].set_frequency(pitch_calc::hz_from_step(note as f32));
        self.oscillators[1].set_frequency(pitch_calc::hz_from_step(note as f32) * 1.005);
        self.oscillators[2].set_frequency(pitch_calc::hz_from_step(note as f32) * 0.995);
        self.envelope.note_on();
    }

    pub fn note_off(&mut self) {
        self.current_note = None;
        self.envelope.note_off();
    }
}

impl AudioProcessor for Voice {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        for oscillator in &mut self.oscillators {
            oscillator.set_sample_rate(settings.sample_rate());
        }
        self.envelope.set_sample_rate(settings.sample_rate());
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        for frame in data.frames_mut() {
            let mut oscillator_value = 0.0;
            for oscillator in &self.oscillators {
                oscillator_value += oscillator.get();
            }

            let envelope_volume = self.envelope.volume();
            let output = self.volume * oscillator_value * envelope_volume;

            for sample in frame.iter_mut() {
                *sample += output;
            }

            self.envelope.tick();
            for oscillator in &mut self.oscillators {
                oscillator.tick();
            }
        }
    }
}
