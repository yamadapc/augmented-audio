/// VST
pub use vst;

/// Run [`audio::processor::AudioProcessor`]s and [`audio::processor::midi::MidiEventHandler`] as
/// stand-alone applications.
pub mod application;
/// Process audio and MIDI information by defining [`audio::processor::AudioProcessor`]s, using
/// [`audio::processor::utility`] built-in processors or building [`audio::processor::graph`]s. Handle
/// real-time shared memory through ref-counting with [`audio::gc`].
pub mod audio;
/// Data-structures
pub mod data;
/// DSP (filters)
pub mod dsp;
/// GUI utilities, subject to change, wrapping [`iced`]
pub mod gui;
/// Operational utilities
pub mod ops;
/// Testing helpers
pub mod testing;
