use basedrop::{Handle, Owned, Shared};
use midir::{MidiInput, MidiInputConnection};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MidiError {
    #[error("Failed to initialize MIDI input")]
    InitError(#[from] midir::InitError),
    #[error("Failed to connect to MIDI input")]
    ConnectError(#[from] midir::ConnectError<MidiInput>),
}

pub type MidiMessageQueue = Shared<atomic_queue::Queue<Owned<MidiMessageWrapper>>>;

pub struct MidiMessageWrapper {
    pub message_data: [u8; 3],
    pub timestamp: u64,
}

struct MidiCallbackContext {
    handle: Handle,
    messages: MidiMessageQueue,
}

impl MidiCallbackContext {
    pub fn new(handle: Handle, messages: MidiMessageQueue) -> Self {
        MidiCallbackContext { messages, handle }
    }
}

fn midi_callback(timestamp: u64, bytes: &[u8], context: &mut MidiCallbackContext) {
    if bytes.len() > 3 {
        log::debug!(
            "Received a 3+ bytes long MIDI message. It'll be ignored. {:?}",
            bytes
        );
        return;
    }

    log::debug!("Handling midi message: {:?}", bytes);
    let mut message_data: [u8; 3] = [0, 0, 0];
    let mut i = 0;
    for b in bytes {
        message_data[i] = *b;
        i += 1;
    }

    let message = Owned::new(
        &context.handle,
        MidiMessageWrapper {
            message_data,
            timestamp,
        },
    );
    context.messages.push(message);
}

pub struct MidiHost {
    handle: Handle,
    connections: Vec<MidiInputConnection<MidiCallbackContext>>,
    current_messages: MidiMessageQueue,
}

impl MidiHost {
    pub fn new(handle: &Handle) -> Self {
        Self {
            handle: handle.clone(),
            connections: Vec::new(),
            current_messages: Shared::new(handle, atomic_queue::Queue::new(100)),
        }
    }

    pub fn messages(&self) -> &MidiMessageQueue {
        &self.current_messages
    }

    pub fn start(&mut self) -> Result<(), MidiError> {
        log::info!("Creating MIDI input");
        let input = midir::MidiInput::new("plugin-host")?;
        log::info!("Connecting to all ports");

        for port in &input.ports() {
            let input = midir::MidiInput::new("plugin-host")?;
            log::info!("MIDI port - {:?}", input.port_name(&port));
            log::info!("Creating MIDI connection");
            let connection = input.connect(
                &port,
                "main-port",
                midi_callback,
                MidiCallbackContext::new(self.handle.clone(), self.current_messages.clone()),
            )?;
            self.connections.push(connection);
        }
        log::info!("Connected to all MIDI ports");

        Ok(())
    }
}

impl Drop for MidiHost {
    fn drop(&mut self) {
        log::info!("Closing MIDI connections");
        while let Some(connection) = self.connections.pop() {
            connection.close();
        }
    }
}
