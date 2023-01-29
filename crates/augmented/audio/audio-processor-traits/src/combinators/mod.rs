pub use function::*;
pub use map::*;
pub use mix::*;

mod function {
    use crate::simple_processor::MonoAudioProcessor;
    use crate::{AudioContext, SimpleAudioProcessor};
    use std::marker::PhantomData;

    pub struct AudioProcessorFunction<F, SampleType> {
        f: F,
        phantom: PhantomData<SampleType>,
    }

    impl<F, SampleType> SimpleAudioProcessor for AudioProcessorFunction<F, SampleType>
    where
        F: FnMut(&mut AudioContext, &mut [SampleType]),
        SampleType: Copy,
    {
        type SampleType = SampleType;

        fn s_process_frame(&mut self, context: &mut AudioContext, frame: &mut [Self::SampleType]) {
            (self.f)(context, frame)
        }
    }

    pub fn processor_function<F, SampleType>(
        f: F,
    ) -> impl SimpleAudioProcessor<SampleType = SampleType>
    where
        F: FnMut(&mut AudioContext, &mut [SampleType]),
        SampleType: Copy,
    {
        AudioProcessorFunction {
            f,
            phantom: PhantomData,
        }
    }

    struct MonoAudioProcessorFunction<F, SampleType> {
        f: F,
        phantom: PhantomData<SampleType>,
    }

    impl<F, SampleType> MonoAudioProcessor for MonoAudioProcessorFunction<F, SampleType>
    where
        F: FnMut(&mut AudioContext, SampleType) -> SampleType,
        SampleType: Copy,
    {
        type SampleType = SampleType;

        fn m_process(
            &mut self,
            context: &mut AudioContext,
            sample: Self::SampleType,
        ) -> Self::SampleType {
            (self.f)(context, sample)
        }
    }

    pub fn mono_processor_function<F, SampleType>(
        f: F,
    ) -> impl MonoAudioProcessor<SampleType = SampleType>
    where
        F: FnMut(&mut AudioContext, SampleType) -> SampleType,
        SampleType: Copy,
    {
        MonoAudioProcessorFunction {
            f,
            phantom: PhantomData,
        }
    }

    pub fn mono_generator_function<F, SampleType>(
        mut f: F,
    ) -> impl MonoAudioProcessor<SampleType = SampleType>
    where
        F: FnMut(&mut AudioContext) -> SampleType,
        SampleType: Copy,
    {
        mono_processor_function(move |context, _sample| f(context))
    }
}

mod map {
    use crate::{AudioBuffer, AudioContext, AudioProcessor};

    struct MapProcessor<P, F> {
        processor: P,
        f: F,
    }

    impl<P, F> AudioProcessor for MapProcessor<P, F>
    where
        P: AudioProcessor,
        F: FnMut(&mut AudioContext, &mut [P::SampleType]),
    {
        type SampleType = P::SampleType;

        fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
            &mut self,
            context: &mut AudioContext,
            output: &mut BufferType,
        ) {
            self.processor.process(context, output);
            for frame in output.frames_mut() {
                (self.f)(context, frame);
            }
        }
    }

    pub fn map_processor<P: AudioProcessor, F: FnMut(&mut AudioContext, &mut [P::SampleType])>(
        processor: P,
        f: F,
    ) -> impl AudioProcessor<SampleType = P::SampleType> {
        MapProcessor { processor, f }
    }
}

mod mix {
    use super::map::map_processor;
    use crate::AudioProcessor;
    use num::{Float, Zero};
    use std::ops::AddAssign;

    pub fn mono<P>(processor: P) -> impl AudioProcessor<SampleType = P::SampleType>
    where
        P: AudioProcessor,
        P::SampleType: Float + AddAssign,
    {
        map_processor(processor, |_ctx, frame| {
            let mut sum = P::SampleType::zero();
            for sample in frame.iter() {
                sum += *sample;
            }
            for sample in frame.iter_mut() {
                *sample = sum;
            }
        })
    }
}
