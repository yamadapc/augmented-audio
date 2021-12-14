# A Rust trait for Audio Processors

In these notes I'll go over the `AudioProcessor` trait, its previous and current iterations and supporting library.

Before starting, I should say I'm not a professional audio developer and therefore my explorations on this area are a
hobby. That means that I'm here to learn and have fun.

## Audio Processing

In audio processing code, we'd like to manipulate audio. Audio is recorded by sampling a signal at a certain rate, such
as 44100Hz. This recording may long and stored in a file, such as a mp3 file of our favorite songs, or it may be very
short to be processed in real-time (for example, a 1ms buffer of samples for applying real-time effects to a signal).

Each sample is a number, usually a float (but sometimes this is an integer). In order to process audio, we make
transformations into these numbers (or generate signals).

When doing real-time audio processing, we will supply a callback to the underlying audio driver which will be called
whenever a buffer is recorded.

For example, if the buffer size is set to 64, at 44100Hz, our callback will get called roughly every 1.4ms with a new
set of samples to process. This callback must finish in 1.4ms or else there will be a dropout in audio. A larger buffer
size may be set to give more time for processing, but this will add latency to the output.

In this context, an audio processing program (excluding the actual IO wiring to set-up the callback), might look
something like this:

```rust
fn process_audio_block(buffer: &mut [f32]) {
    // ...
}
```

This is very straightforward and often how things look like. The audio processing logic is a function which takes a
mutable slice of samples.

However, there are a number of concerns that aren't handled above:

**Processor State**: Audio processing logic will want to maintain state, for example, values of parameters being changed
through a UI or a longer buffer containing previous samples.

**Sample buffer layout**: Audio is often multi-channel (2 channels or more) and different audio APIs expect/provide
audio with different layouts.

For example, the slice could look like `[left_sample0, right_sample0, left_sample1, right_sample1, ...]`
or it could look like `[left_sample0, left_sample1, ..., right_sample0, right_sample1, ...]`. Or there could be no
single slice and instead multiple slices making an audio buffer a collection of pointers to each channel's slice.

**Sample type**: Samples may be in different formats, for example they could be `f32` or `f64` or they could be
integers.

**Sample rate / context**: Often an audio processor needs to know the sample rate or other context information to
operate.

Audio related information might include sampling rate and nº of channels. Context might include the play-head position
of a track or the current tempo in BPM.