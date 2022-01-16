# audio-processor-standalone-midi
[![crates.io](https://img.shields.io/crates/v/audio-processor-standalone-midi.svg)](https://crates.io/crates/audio-processor-standalone-midi)
[![docs.rs](https://docs.rs/audio-processor-standalone-midi/badge.svg)](https://docs.rs/audio-processor-standalone-midi/)
- - -

Wraps `midir` to provide MIDI input handling. The host may be started with `MidiHost`.

When MIDI messages are received, they're pushed onto a lock-free `atomic_queue::Queue`. The messages are picked-up in
the audio-thread by `MidiAudioThreadHandler`.

It provides easy stand-alone MIDI integration with:

* `audio-processor-traits` - `MidiEventHandler` trait
* `rust-vst` - `PluginInstance`

This crate provides conversion into the VST types, which is to allow a VST host to use it. This is provided by the
`MidiVSTConverter`.

Currently, MIDI messages over 3 bytes are dropped by this host. In addition, the queue is bounded & a size must be
provided. `Default` implementations will use a MIDI queue capacity of 100.

## Memory Safety
In order to integrate with the VST C API, this crate does manual memory allocation and handling. This is due to VST
event types being unrepresentable as safe Rust constructs (and due to real-time safety being required as well).

## Real-time Safety
This crate provides the `host` side, which is the MIDI host. This host allocates when it receives messages from `midir`.

The events are forwarded onto a lock-free queue (`atomic_queue`).

On the `audio_thread` and `vst` modules, past construction methods that should be called on the audio-thread will not
(de)-allocate. This is tested using the `assert_no_alloc` crate.

In addition, `basedrop` / `audio_garbage_collector` are used to prevent de-allocation from happening on the
audio-thread.

## Test coverage
The crate has unit-tests (though it should be considered experimental as all of this repository).

Test coverage is at 80%.

## Actix & Actors
The `MidiHost` exposes an actix API, to be used with the actix actor system. This is to ease communicating with the midi
handler from multiple threads.