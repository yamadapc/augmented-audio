//! Provides abstractions for implementing:
//!
//! * Audio processing nodes
//! * MIDI processing nodes
//! * Audio buffers
//!
//! An audio processor implemented with these traits may work with multiple sample types, audio
//! buffer types and audio processing back-ends.
//!
//! Start looking at [AudioProcessor], then have a look at [AudioBuffer] and [MidiEventHandler].
use std::marker::PhantomData;

pub use num::Float;

pub use atomic_float::AtomicF32;
pub use audio_buffer::{AudioBuffer, InterleavedAudioBuffer};
pub use midi::{MidiEventHandler, MidiMessageLike};

/// Atomic F32 implementation with `num` trait implementations
pub mod atomic_float;
/// Provides an abstraction for audio buffers that works for CPAL and VST layouts
pub mod audio_buffer;
/// Provides an abstraction for MIDI processing that works for stand-alone and VST events
pub mod midi;

/// Options provided to the audio-processor before calling `process`.
#[derive(Clone, PartialEq, Debug, Copy)]
pub struct AudioProcessorSettings {
    pub sample_rate: f32,
    pub input_channels: usize,
    pub output_channels: usize,
    pub block_size: usize,
}

impl Default for AudioProcessorSettings {
    fn default() -> Self {
        Self::new(44100.0, 2, 2, 512)
    }
}

impl AudioProcessorSettings {
    pub fn new(
        sample_rate: f32,
        input_channels: usize,
        output_channels: usize,
        block_size: usize,
    ) -> Self {
        AudioProcessorSettings {
            sample_rate,
            input_channels,
            output_channels,
            block_size,
        }
    }

    /// The sample rate in samples/second as a floating point number
    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    /// The number of input channels
    pub fn input_channels(&self) -> usize {
        self.input_channels
    }

    /// The number of output channels
    pub fn output_channels(&self) -> usize {
        self.output_channels
    }

    /// The number of samples which will be provided on each `process` call
    pub fn block_size(&self) -> usize {
        self.block_size
    }
}

impl AudioProcessorSettings {
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }

    pub fn set_input_channels(&mut self, input_channels: usize) {
        self.input_channels = input_channels;
    }

    pub fn set_output_channels(&mut self, output_channels: usize) {
        self.output_channels = output_channels;
    }

    pub fn set_block_size(&mut self, block_size: usize) {
        self.block_size = block_size;
    }
}

/// Represents an audio processing node.
///
/// Implementors should define the SampleType the node will work over. See some [examples here](https://github.com/yamadapc/augmented-audio/tree/master/crates/audio-processor-standalone/examples).
pub trait AudioProcessor {
    type SampleType;

    /// Prepare for playback based on current audio settings
    fn prepare(&mut self, _settings: AudioProcessorSettings) {}

    /// Process a block of samples by mutating the input `AudioBuffer`
    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    );
}

/// Auto-implemented object version of the audio-processor trait.
///
/// Given a known buffer-type, audio-processors can be made into objects using this type.
pub trait ObjectAudioProcessor<BufferType> {
    fn prepare_obj(&mut self, _settings: AudioProcessorSettings) {}
    fn process_obj(&mut self, data: &mut BufferType);
}

impl<SampleType, BufferType, Processor> ObjectAudioProcessor<BufferType> for Processor
where
    SampleType: Float + Send,
    BufferType: AudioBuffer<SampleType = SampleType>,
    Processor: AudioProcessor<SampleType = SampleType>,
{
    fn prepare_obj(&mut self, settings: AudioProcessorSettings) {
        <Processor as AudioProcessor>::prepare(self, settings);
    }

    fn process_obj(&mut self, data: &mut BufferType) {
        <Processor as AudioProcessor>::process(self, data);
    }
}

/// An audio-processor which doesn't do any work.
pub struct NoopAudioProcessor<SampleType>(PhantomData<SampleType>);

impl<SampleType: Send> AudioProcessor for NoopAudioProcessor<SampleType> {
    type SampleType = SampleType;

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        _data: &mut BufferType,
    ) {
    }
}

/// An audio-processor which mutes all channels.
pub struct SilenceAudioProcessor<SampleType>(PhantomData<SampleType>);

impl<SampleType> SilenceAudioProcessor<SampleType> {
    pub fn new() -> Self {
        SilenceAudioProcessor(PhantomData)
    }
}

impl<SampleType> Default for SilenceAudioProcessor<SampleType> {
    fn default() -> Self {
        Self::new()
    }
}

impl<SampleType: Float + Send> AudioProcessor for SilenceAudioProcessor<SampleType> {
    type SampleType = SampleType;

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        output: &mut BufferType,
    ) {
        for sample in output.slice_mut() {
            *sample = SampleType::zero();
        }
    }
}
