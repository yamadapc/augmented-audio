use std::ops::Deref;

use basedrop::{Handle, Owned, Shared};
use midir::{MidiInput, MidiInputConnection};
use thiserror::Error;

use atomic_queue::Queue;
use audio_processor_traits::MidiMessageLike;

use crate::constants::MIDI_BUFFER_CAPACITY;

/// Host for MIDI messages, opens all ports & forwards them onto a lock-free queue the audio-thread
/// can pop from.
///
/// The host will close all MIDI connections on drop.
pub struct MidiHost {
    handle: Handle,
    connections: Vec<MidiInputConnection<MidiCallbackContext>>,
    current_messages: MidiMessageQueue,
}

impl MidiHost {
    /// Create the host, linked to GC `Handle` and with queue `capacity` of messages.
    pub fn new(handle: &Handle, capacity: usize) -> Self {
        Self {
            handle: handle.clone(),
            connections: Vec::new(),
            current_messages: Shared::new(handle, Queue::new(capacity)),
        }
    }

    /// Create the host with default 100 capacity
    pub fn default_with_handle(handle: &Handle) -> Self {
        MidiHost::new(handle, MIDI_BUFFER_CAPACITY)
    }

    /// Get a reference to the message queue
    pub fn messages(&self) -> &MidiMessageQueue {
        &self.current_messages
    }

    /// Start the MIDI connections
    pub fn start(&mut self) -> Result<(), MidiError> {
        log::info!("Creating MIDI input `plugin-host`");
        let input = midir::MidiInput::new("plugin-host")?;

        for port in &input.ports() {
            let input = midir::MidiInput::new("plugin-host")?;
            log::info!("MIDI input: {:?}", input.port_name(&port));
            let connection = input.connect(
                &port,
                "main-port",
                midi_callback,
                MidiCallbackContext::new(self.handle.clone(), self.current_messages.clone()),
            )?;
            self.connections.push(connection);
        }

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

/// An error during MIDI processing
#[derive(Error, Debug)]
pub enum MidiError {
    #[error("Failed to initialize MIDI input")]
    InitError(#[from] midir::InitError),
    #[error("Failed to connect to MIDI input")]
    ConnectError(#[from] midir::ConnectError<MidiInput>),
}

/// Shared reference to the MIDI message queue
pub type MidiMessageQueue = Shared<Queue<MidiMessageEntry>>;

/// One entry in the MIDI message queue
pub struct MidiMessageEntry(pub Owned<MidiMessageWrapper>);

impl Deref for MidiMessageEntry {
    type Target = Owned<MidiMessageWrapper>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl MidiMessageLike for MidiMessageEntry {
    fn is_midi(&self) -> bool {
        true
    }

    fn bytes(&self) -> Option<&[u8]> {
        Some(&self.message_data)
    }
}

/// A wrapper type to wrap messages. Messages must be 3 bytes in length (SysEx will be dropped).
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
        MidiCallbackContext { handle, messages }
    }
}

fn midi_callback(timestamp: u64, bytes: &[u8], context: &mut MidiCallbackContext) {
    if bytes.len() > 3 {
        log::trace!(
            "Received a 3+ bytes long MIDI message. It'll be ignored. {:?}",
            bytes
        );
        return;
    }

    log::trace!("Handling midi message: {:?}", bytes);
    let mut message_data: [u8; 3] = [0, 0, 0];
    for (i, b) in bytes.iter().enumerate() {
        message_data[i] = *b;
    }

    let message = MidiMessageEntry(Owned::new(
        &context.handle,
        MidiMessageWrapper {
            message_data,
            timestamp,
        },
    ));
    context.messages.push(message);
}
