use std::ops::Deref;

use actix::{Actor, Context, Handler, Message, MessageResponse, Supervised, SystemService};
use basedrop::{Handle, Owned, Shared};
use thiserror::Error;

use atomic_queue::Queue;
use audio_processor_traits::MidiMessageLike;
use midir::os::unix::VirtualInput;
use midir::{MidiInput, MidiInputConnection};

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

    /// Build a MidiHost with a pre-built queue
    pub fn default_with_queue(handle: &Handle, queue: MidiMessageQueue) -> Self {
        Self {
            handle: handle.clone(),
            connections: Vec::new(),
            current_messages: queue,
        }
    }

    /// Get a reference to the message queue
    pub fn messages(&self) -> &MidiMessageQueue {
        &self.current_messages
    }

    /// Start the MIDI connections
    pub fn start_midi(&mut self) -> Result<(), MidiError> {
        log::info!("Creating MIDI input `audio_processor_standalone_midi`");
        let input = midir::MidiInput::new("audio_processor_standalone_midi")?;

        let virtual_port_name = Self::virtual_port_name();
        log::info!("Creating virtual MIDI input `{}`", virtual_port_name);
        let virtual_input = input.create_virtual(
            &*virtual_port_name,
            midi_callback,
            MidiCallbackContext::new(self.handle.clone(), self.current_messages.clone()),
        )?;
        self.connections.push(virtual_input);

        let input = midir::MidiInput::new("audio_processor_standalone_midi")?;
        for port in &input.ports() {
            let input = midir::MidiInput::new("audio_processor_standalone_midi")?;
            log::debug!("MIDI input: {:?}", input.port_name(port));
            let connection = input.connect(
                port,
                "main-port",
                midi_callback,
                MidiCallbackContext::new(self.handle.clone(), self.current_messages.clone()),
            )?;
            self.connections.push(connection);
        }

        Ok(())
    }

    /// For integration testing purposes `MidiHost` starts a virtual input which it'll also connect
    /// into. This input contains the PID of the host process.
    ///
    /// Generating this name dynamically avoids flakiness in integration tests.
    pub fn virtual_port_name() -> String {
        let virtual_port_name = format!("audio_processor_standalone_midi_{}", std::process::id());
        virtual_port_name
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

impl Actor for MidiHost {
    type Context = Context<Self>;
}

impl Default for MidiHost {
    fn default() -> Self {
        Self::default_with_handle(audio_garbage_collector::handle())
    }
}

impl Supervised for MidiHost {}

impl SystemService for MidiHost {
    fn service_started(&mut self, _ctx: &mut Context<Self>) {
        log::info!("MidiHost started");
    }
}

#[derive(Message)]
#[rtype(result = "Result<(), MidiError>")]
pub struct StartMessage;

impl Handler<StartMessage> for MidiHost {
    type Result = Result<(), MidiError>;

    fn handle(&mut self, _msg: StartMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.start_midi()
    }
}

#[derive(Message)]
#[rtype(result = "GetQueueMessageResult")]
pub struct GetQueueMessage;

#[derive(Message, MessageResponse)]
#[rtype(result = "()")]
pub struct GetQueueMessageResult(pub MidiMessageQueue);

impl Handler<GetQueueMessage> for MidiHost {
    type Result = GetQueueMessageResult;

    fn handle(&mut self, _msg: GetQueueMessage, _ctx: &mut Self::Context) -> Self::Result {
        GetQueueMessageResult(self.current_messages.clone())
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

#[cfg(test)]
mod test {
    use assert_no_alloc::assert_no_alloc;

    use audio_garbage_collector::make_shared;

    use super::*;

    #[test]
    fn test_create_midi_host() {
        let _host = MidiHost::new(audio_garbage_collector::handle(), 10);
    }

    #[test]
    fn test_create_default_midi_host() {
        let _host = MidiHost::default_with_handle(audio_garbage_collector::handle());
    }

    #[test]
    fn test_create_default_midi_host_with_queue() {
        let queue = make_shared(Queue::new(10));
        let _host = MidiHost::default_with_queue(audio_garbage_collector::handle(), queue);
    }

    #[test]
    fn test_get_message_queue() {
        let queue = make_shared(Queue::new(10));
        let host = MidiHost::default_with_queue(audio_garbage_collector::handle(), queue.clone());
        assert_eq!(
            host.messages().clone().deref() as *const Queue<MidiMessageEntry>,
            queue.deref() as *const Queue<MidiMessageEntry>
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_start_midi_callback() {
        let mut host = MidiHost::default();
        host.start_midi().unwrap();
    }

    #[test]
    fn test_midi_callback_on_long_message_drops_message() {
        let queue = make_shared(Queue::new(10));
        let mut context =
            MidiCallbackContext::new(audio_garbage_collector::handle().clone(), queue.clone());
        let bytes: [u8; 4] = [10, 20, 30, 40];

        assert_no_alloc(|| {
            midi_callback(0, &bytes, &mut context);
        });

        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_midi_callback_on_message() {
        let queue = make_shared(Queue::new(10));
        let mut context =
            MidiCallbackContext::new(audio_garbage_collector::handle().clone(), queue.clone());
        let bytes: [u8; 3] = [10, 20, 30];

        // assert_allocation_count(1, || {
        midi_callback(0, &bytes, &mut context);
        // });
        assert_eq!(queue.len(), 1);
        let msg = queue.pop().unwrap();
        assert_eq!(msg.message_data, bytes);
    }

    #[cfg(target_os = "macos")]
    #[actix::test]
    async fn test_midi_handler_actor_start() {
        let source_queue = make_shared(Queue::new(10));
        let addr =
            MidiHost::default_with_queue(audio_garbage_collector::handle(), source_queue.clone())
                .start();
        let _result = addr.send(StartMessage).await.unwrap().unwrap();
        let GetQueueMessageResult(queue) = addr.send(GetQueueMessage).await.unwrap();
        assert_eq!(
            queue.deref() as *const Queue<MidiMessageEntry>,
            source_queue.deref() as *const Queue<MidiMessageEntry>
        );
    }
}
