pub trait MidiMessage {
    fn bytes(&self) -> &[u8];
}

pub trait MidiEventHandler<Message: MidiMessage> {
    fn process_midi(&mut self, midi_messages: &[Message]);
}
