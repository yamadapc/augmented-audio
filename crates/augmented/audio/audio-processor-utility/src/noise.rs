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

use rand::{Rng, SeedableRng};

use audio_processor_traits::simple_processor::MonoAudioProcessor;
use audio_processor_traits::{AudioContext, Float};

/// White-noise generator, returns random numbers between -1 and 1 using [`rand::rngs::SmallRng`].
///
/// Float type for samples is generic, requires implementations of [`num::Float`] and
/// [`rand::distributions::uniform::SampleUniform`]
///
/// # Example
///
/// Generating sample by sample see [`audio_processor_traits::SimpleAudioProcessor`]:
///
/// ```
/// use audio_processor_traits::{AudioContext, AudioProcessorSettings};
/// use audio_processor_traits::simple_processor::MonoAudioProcessor;
/// use audio_processor_utility::noise::WhiteNoiseProcessor;
///
/// let mut context = AudioContext::from(AudioProcessorSettings::default());
/// let mut  processor = WhiteNoiseProcessor::<f32>::default();
/// let noise1 = processor.m_process(&mut context, 0.0);
/// let noise2 = processor.m_process(&mut context, 0.0);
/// let noise3 = processor.m_process(&mut context, 0.0);
/// ```
///
/// Generating buffer by buffer see [`audio_processor_traits::simple_processor::process_buffer`]:
///
/// ```
/// use audio_processor_traits::simple_processor::process_buffer;
/// use audio_processor_traits::{AudioContext, AudioProcessorSettings, AudioBuffer};
/// use audio_processor_utility::noise::WhiteNoiseProcessor;
///
/// let mut context = AudioContext::from(AudioProcessorSettings::default());
/// let mut buffer = AudioBuffer::empty();
/// buffer.resize(2, 1000);
/// let mut  processor = WhiteNoiseProcessor::<f32>::default();
/// process_buffer(&mut context, &mut processor, &mut buffer);
/// ```
///
/// Using as a standalone processor see [`audio_processor_standalone`]:
///
/// ```
/// use audio_processor_standalone::StandaloneAudioOnlyProcessor;
/// use audio_processor_traits::simple_processor::MonoCopyProcessor;
/// use audio_processor_utility::noise::WhiteNoiseProcessor;
///
/// let processor = MonoCopyProcessor::new(WhiteNoiseProcessor::<f32>::default());
/// let _standalone = StandaloneAudioOnlyProcessor::new(processor, Default::default());
/// // now call standalone_start
/// ```
pub struct WhiteNoiseProcessor<SampleType> {
    rng: rand::rngs::SmallRng,
    phantom: PhantomData<SampleType>,
}

impl<SampleType> Default for WhiteNoiseProcessor<SampleType> {
    fn default() -> Self {
        Self {
            rng: rand::rngs::SmallRng::from_entropy(),
            phantom: PhantomData::default(),
        }
    }
}

impl<SampleType: Float + rand::distributions::uniform::SampleUniform> MonoAudioProcessor
    for WhiteNoiseProcessor<SampleType>
{
    type SampleType = SampleType;

    fn m_process(
        &mut self,
        _context: &mut AudioContext,
        _sample: Self::SampleType,
    ) -> Self::SampleType {
        self.rng.gen_range(-SampleType::one()..SampleType::one())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_no_alloc() {
        let mut context = AudioContext::default();
        let mut processor = WhiteNoiseProcessor::default();
        assert_no_alloc::assert_no_alloc(|| {
            for i in 0..10 {
                processor.m_process(&mut context, i as f32);
            }
        })
    }
}
