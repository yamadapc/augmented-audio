# Audio Processor Traits
[![crates.io](https://img.shields.io/crates/v/audio-processor-traits.svg)](https://crates.io/crates/audio-processor-traits)
[![docs.rs](https://docs.rs/audio-processor-traits/badge.svg)](https://docs.rs/audio-processor-traits/)
- - -
Traits for audio processor types and audio buffer types. Heavily subject to change.

## Primer
This is very much exploration based and I'm still finding out the best API to express in Rust types.

I think I've found a good abstraction to handle **AudioBuffer** and basic FX **AudioProcessor**, but there's quite a
lot more to uncover.

## Motivation
It'd be great to have generic traits for structs that process audio.

These may be 1 or more abstractions on top of audio processing which enable higher-level systems to manipulate audio
processing nodes.

At its core, audio processing may be defined as a function:
```rust
fn process_audio(_input: &mut [f32]) { /* ... */ }
```

`input` would be your input/output buffer and you could perform mutable operations on it. However, this does not
encode several audio processing concerns, such as:

* The buffer won't be always `f32`, it might be `f64`
* The buffer channel count isn't expressed
* The buffer layout for multi-channel data isn't expressed (e.g. Interleaved data or otherwise)
* Sample rate isn't expressed
* Dry/wet configurations aren't expressed
* MIDI & other concerns aren't expressed

We'd like some abstraction that covers some of these issues. Without thinking about external system problems (such as
MIDI, state & dry/wet), a basic audio processor trait can solve buffer/sample conversion issues.

## AudioBuffer

The first part is the `AudioBuffer` trait.
```rust
pub trait AudioBuffer {
    type SampleType: num::Float + Sync + Send;

    /// The number of channels in this buffer
    fn num_channels(&self) -> usize;

    /// The number of samples in this buffer
    fn num_samples(&self) -> usize;

    /// Get a ref to an INPUT sample in this buffer
    fn get(&self, channel: usize, sample: usize) -> &Self::SampleType;

    /// Get a mutable ref to an OUTPUT sample in this buffer
    ///
    /// On some implementations this may yield a different value than `.get`.
    fn get_mut(&mut self, channel: usize, sample: usize) -> &mut Self::SampleType;

    /// Set an OUTPUT sample in this buffer
    fn set(&mut self, channel: usize, sample: usize, value: Self::SampleType);

    /// Create a read only iterator
    fn iter(&self) -> AudioBufferIterator<Self> {
        AudioBufferIterator::new(&self)
    }
}
```

It provides an abstraction to get the size of the buffer and modify it as well as a helper read-only iterator
implementation.

There're 3 main implementations of this trait in this crate.

### InterleavedAudioBuffer
This implementation provides compatibility with interleaved buffers, where multi-channel samples are interleaved with
one another.

This provides compatibility of the `AudioBuffer` trait with `cpal`.

### VSTAudioBuffer
This implementation provides compatibility with the VST API and the `rust-vst`/`vst` crate.

In `vst`, the channels are separate slices of continuous samples & input/output are separate pointers.

### SliceAudioBuffer
This is unused at the moment, but provides support for other audio buffers that have similar layout than VST, where
each channel is a slice of floats.

## AudioProcessor

The **AudioProcessor** trait is only two methods:

```rust
pub trait AudioProcessor: Send + Sync {
    type SampleType;
    fn prepare(&mut self, _settings: AudioProcessorSettings) {}
    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    );
}
```

It provides a prepare callback, where channel & sample rate configuration will be provided and a process callback where
a generic `AudioBuffer` is provided.

### Design notes
#### SampleType associated type

The `SampleType` is provided as an associated type to both the `AudioBuffer` and the `AudioProcessor` traits. This
enables implementors to use generic `SampleType` types in their processors.

For example, this is the `SilenceAudioProcessor` implementation in this crate, which should work for any `num::Float`
type and any `AudioBuffer` implementation:

```rust
pub struct SilenceAudioProcessor<SampleType>(PhantomData<SampleType>);

impl<SampleType: num::Float + Send + Sync> AudioProcessor for SilenceAudioProcessor<SampleType> {
    type SampleType = SampleType;

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        output: &mut BufferType,
    ) {
        for sample_index in 0..output.num_samples() {
            for channel_index in 0..output.num_channels() {
                output.set(
                    channel_index,
                    sample_index,
                    <BufferType as AudioBuffer>::SampleType::zero(),
                );
            }
        }
    }
}
```

## Pending work
* ~~MIDI trait~~
* Richer API for applications
* State management guidelines, using a background ref-counting garbage-collector & immutable 'state handle' references
  (while still allowing the internal state of a processor to be mutable)
* Automatic implementation of the VST API for all trait implementors
* Automatic implementation of the LV2 API for all trait implementors
* Automatic implementation of a "stand-alone" `cpal` based App for all trait implementors (see
  `audio-processor-standalone` in this repository)
* An audio-graph implementation
* GUI support
* Testing tools