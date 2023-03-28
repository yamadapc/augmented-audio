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

use std::marker::PhantomData;

use crate::simple_processor::MonoAudioProcessor;
use crate::{AudioContext, SimpleAudioProcessor};

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

pub fn processor_function<F, SampleType>(f: F) -> impl SimpleAudioProcessor<SampleType = SampleType>
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_processor_function() {
        let mut processor = processor_function(|_context, frame: &mut [f32]| {
            for channel in frame {
                *channel *= 2.0;
            }
        });

        let mut ctx = AudioContext::default();
        let mut slice = [2.0, 4.0];
        processor.s_process_frame(&mut ctx, &mut slice);
        assert_eq!(slice, [4.0, 8.0]);
    }

    #[test]
    fn test_mono_processor_function() {
        let mut processor =
            mono_processor_function(|_context, sample: f32| -> f32 { sample * 2.0 });

        let mut ctx = AudioContext::default();
        let result = processor.m_process(&mut ctx, 2.0);
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_mono_generator_function() {
        let mut processor = mono_generator_function(|_context| -> f32 { 2.0 });

        let mut ctx = AudioContext::default();
        let result = processor.m_process(&mut ctx, 0.0);
        assert_eq!(result, 2.0);
    }
}
