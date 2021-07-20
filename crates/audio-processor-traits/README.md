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
An `AudioBuffer` trait is provided. It provides an abstraction to get the size of the buffer and modify it.

The `AudioBuffer` trait may wrap samples in different layouts or with different ownership, however, it's recommended to
process samples using the `AudioBuffer::frame`, `AudioBuffer::frame_mut`, `AudioBuffer::slice` and
`AudioBuffer::slice_mut`.

The reason for this is that using `slice` iterators is much more efficient than iterating over a range of numbers and
calling `AudioBuffer::get`.

## AudioProcessor

The **AudioProcessor** trait is only two methods:

```rust
pub trait AudioProcessor {
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

impl<SampleType: num::Float + Send> AudioProcessor for SilenceAudioProcessor<SampleType> {
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

## Buffer performance
On a trivial gain benchmark, performance using the `AudioBuffer::get` APIs is between 10-20x worse on a 
`VecAudioBuffer` than a `Vec`.

Other things to measure:

* Measure buffer conversion time on a small window size
  * Compare this with overhead of the `AudioBuffer` abstraction
* Measure overhead in contrast to the grand scheme of a non-trivial processor (not gain)

### Other thoughts
* Which is the more efficient layout?
* Should audio-processors expose a `process` function that works sample by sample? Perhaps this is easier to optimise

## License
MIT
