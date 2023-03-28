pub use num;
pub use num::Float;
pub use num::Zero;

pub use atomic_float::{AtomicF32, AtomicF64};
pub use audio_buffer::{AudioBuffer, InterleavedAudioBuffer, OwnedAudioBuffer, VecAudioBuffer};
pub use context::AudioContext;
pub use midi::{MidiEventHandler, MidiMessageLike, NoopMidiEventHandler};
pub use noop_processors::*;
pub use settings::*;
pub use simple_processor::{BufferProcessor, SimpleAudioProcessor};

/// Atomic F32 implementation with `num` trait implementations
pub mod atomic_float;
/// Provides an abstraction for audio buffers that works for [`cpal`] and [`vst`] layouts
pub mod audio_buffer;
pub mod combinators;
/// The "staged context" for audio processors
pub mod context;
/// Provides an abstraction for MIDI processing that works for stand-alone and [`vst`] events
pub mod midi;
/// Parameters for [`AudioProcessor`]
pub mod parameters;
/// Simpler audio processor trait, ingesting sample by sample
pub mod simple_processor;

mod noop_processors;
mod settings;

/// Represents an audio processing node.
///
/// Implementors should define the SampleType the node will work over. See some [examples here](https://github.com/yamadapc/augmented-audio/tree/master/crates/augmented/application/audio-processor-standalone/examples).
pub trait AudioProcessor {
    type SampleType;

    /// Prepare for playback based on current audio settings
    fn prepare(&mut self, _context: &mut AudioContext, _settings: AudioProcessorSettings) {}

    /// Process a block of samples by mutating the input `AudioBuffer`
    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        _context: &mut AudioContext,
        data: &mut BufferType,
    );
}

/// Auto-implemented object version of the audio-processor trait.
///
/// Given a known buffer-type, audio-processors can be made into objects using this type.
pub trait SliceAudioProcessor {
    fn prepare_slice(&mut self, _context: &mut AudioContext, _settings: AudioProcessorSettings) {}
    fn process_slice(&mut self, _context: &mut AudioContext, num_channels: usize, data: &mut [f32]);
}

impl<Processor> SliceAudioProcessor for Processor
where
    Processor: AudioProcessor<SampleType = f32>,
{
    fn prepare_slice(&mut self, context: &mut AudioContext, settings: AudioProcessorSettings) {
        <Processor as AudioProcessor>::prepare(self, context, settings);
    }

    fn process_slice(&mut self, context: &mut AudioContext, num_channels: usize, data: &mut [f32]) {
        let mut buffer = InterleavedAudioBuffer::new(num_channels, data);
        <Processor as AudioProcessor>::process(self, context, &mut buffer);
    }
}

/// Auto-implemented object version of the audio-processor trait.
///
/// Given a known buffer-type, audio-processors can be made into objects using this type.
pub trait ObjectAudioProcessor<BufferType> {
    fn prepare_obj(&mut self, _context: &mut AudioContext, _settings: AudioProcessorSettings) {}
    fn process_obj(&mut self, context: &mut AudioContext, data: &mut BufferType);
}

impl<SampleType, BufferType, Processor> ObjectAudioProcessor<BufferType> for Processor
where
    SampleType: Float + Send,
    BufferType: AudioBuffer<SampleType = SampleType>,
    Processor: AudioProcessor<SampleType = SampleType>,
{
    fn prepare_obj(&mut self, context: &mut AudioContext, settings: AudioProcessorSettings) {
        <Processor as AudioProcessor>::prepare(self, context, settings);
    }

    fn process_obj(&mut self, context: &mut AudioContext, data: &mut BufferType) {
        <Processor as AudioProcessor>::process(self, context, data);
    }
}
