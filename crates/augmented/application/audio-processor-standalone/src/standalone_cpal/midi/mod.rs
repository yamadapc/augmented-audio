#[cfg(not(target_os = "ios"))]
pub use general::*;
#[cfg(target_os = "ios")]
pub use ios::*;

#[cfg(not(target_os = "ios"))]
mod general;
#[cfg(target_os = "ios")]
mod ios;

pub struct MidiContext {
    #[cfg(not(target_os = "ios"))]
    midi_message_queue: MidiMessageQueue,
    #[cfg(not(target_os = "ios"))]
    midi_audio_thread_handler: MidiAudioThreadHandler,
}
