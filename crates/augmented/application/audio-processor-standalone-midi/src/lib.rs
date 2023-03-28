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
//! [![crates.io](https://img.shields.io/crates/v/audio-processor-standalone-midi.svg)](https://crates.io/crates/audio-processor-standalone-midi)
//! [![docs.rs](https://docs.rs/audio-processor-standalone-midi/badge.svg)](https://docs.rs/audio-processor-standalone-midi/)
//! - - -
//!
//! This crate provides conversion into the VST types, which is to allow a VST host to use it. This is provided by the
//! `MidiVSTConverter`.
//!
//! Wraps `midir` to provide MIDI input handling. The host may be started with `MidiHost`.
//!
//! When MIDI messages are received, they're pushed onto a lock-free `atomic_queue::Queue`. The messages are picked-up in
//! the audio-thread by `MidiAudioThreadHandler`.
//!
//! It provides easy stand-alone MIDI integration with:
//!
//! * `audio-processor-traits` - `MidiEventHandler` trait
//! * `rust-vst` - `PluginInstance`
//!
//! Currently, MIDI messages over 3 bytes are dropped by this host. In addition, the queue is bounded & a size must be
//! provided. `Default` implementations will use a MIDI queue capacity of 100.
//! This is a stand-alone MIDI host with helpers for doing real-time handling of MIDI messages in
//! the audio-thread and to integrate with VST plugins.
//!
//! This is part of https://github.com/yamadapc/augmented-audio.
//!
//! # Memory Safety
//! In order to integrate with the VST C API, this crate does manual memory allocation and handling. This is due to VST
//! event types being unrepresentable as safe Rust constructs (and due to real-time safety being required as well).
//!
//! # Real-time Safety
//! This crate provides the `host` side, which is the MIDI host. This host allocates when it receives messages from `midir`.
//!
//! The events are forwarded onto a lock-free queue (`atomic_queue`).
//!
//! On the `audio_thread` and `vst` modules, past construction methods that should be called on the audio-thread will not
//! (de)-allocate. This is tested using the `assert_no_alloc` crate.
//!
//! In addition, `basedrop` / `audio_garbage_collector` are used to prevent de-allocation from happening on the
//! audio-thread.
//!
//! # Test coverage
//! The crate has unit-tests (though it should be considered experimental as all of this repository).
//!
//! Test coverage is at 80%.
//!
//! # Actix & Actors
//! The `MidiHost` exposes an actix API, to be used with the actix actor system. This is to ease communicating with the midi
//! handler from multiple threads.
//!
//! # Usage
//! To use this crate:
//!
//! * [`basedrop::Collector`] You'll need to set-up `basedrop` or `audio-garbage-collector`
//! * [`host::MidiHost`] should be created
//!   - This will connect to inputs & push messages to a queue
//! * [`audio_thread::MidiAudioThreadHandler`] On your audio thread you should pop from the queue
//!   - This is enough to add MIDI to a standalone [`audio_processor_traits::MidiEventHandler`]
//! * [`vst::MidiVSTConverter`] If you're implementing a host, you'll have to convert messages onto
//!   the VST API
//!
//! ```
//! fn example() {
//!     use audio_processor_standalone_midi::audio_thread::MidiAudioThreadHandler;
//!     use audio_processor_standalone_midi::host::MidiHost;
//!     use audio_processor_standalone_midi::vst::MidiVSTConverter;
//!     use basedrop::Collector;
//!
//!     // GC ======================================================================================
//!     // See `audio-garbage-collector` for an easier to use wrapper on top of this.
//!     //
//!     // `Collector` will let us use reference counted values on the audio-thread, which will be
//!     // pushed to a queue for de-allocation. You must set-up a background thread that forces it
//!     // to actually collect garbage from the queue.
//!     let gc = Collector::new();
//!     let handle = gc.handle();
//!
//!     // Host ====================================================================================
//!     // A host may be created with `new` or `default_with_handle`.
//!     let mut host = MidiHost::default_with_handle(&handle);
//!     // It'll connect to all MIDI input ports when started
//!     // host.start().expect("Failed to connect");
//!     // The host will push messages onto a lock-free queue. This is a reference counted value.
//!     let midi_messages_queue = host.messages().clone();
//!
//!     // Audio-thread ============================================================================
//!     // ...
//!     // Within your audio-thread
//!     // ...
//!     // You'll want to share a `MidiAudioThreadHandler` between process calls as it pre-allocates
//!     // buffers.
//!     let mut midi_audio_thread_handler = MidiAudioThreadHandler::default();
//!
//!     // On each tick you'll call collect
//!     midi_audio_thread_handler.collect_midi_messages(&midi_messages_queue);
//!     // You'll get the MIDI message buffer
//!     let midi_messages = midi_audio_thread_handler.buffer();
//!     // ^ This is a `&Vec<MidiMessageEntry>`. If you're using `audio-processor-traits`, you can
//!     //   pass this into any `MidiEventHandler` implementor.
//!
//!     // VST interop =============================================================================
//!     // If you want to interface with a VST plugin, you'll need to convert messages into the
//!     // C-api.
//!     // ...
//!     // You'll want to share this between process calls as it pre-allocates buffers.
//!     let mut midi_vst_converter = MidiVSTConverter::default();
//!     midi_vst_converter.accept(&midi_messages);
//!     // This can be passed into VST plugins.
//!     let events: &vst::api::Events = midi_vst_converter.events();
//! }
//! ```

/// Audio-thread handling of messages
pub mod audio_thread;
/// Defaults
pub mod constants;
/// Hosting of MIDI
pub mod host;

#[cfg(feature = "vst")]
/// VST API conversion
pub mod vst;

#[cfg(all(test, debug_assertions))]
pub(crate) mod test_util;
