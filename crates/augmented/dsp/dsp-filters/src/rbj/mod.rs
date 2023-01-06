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
//! Filters from <https://shepazu.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html>
//!
//! Ported from [vinniefalco/DSPFilters](https://github.com/vinniefalco/DSPFilters/)
use std::fmt::Debug;

use num::pow::Pow;
use num::traits::FloatConst;
use num::Float;

use audio_processor_traits::parameters::{
    make_handle_ref, AudioProcessorHandleProvider, AudioProcessorHandleRef,
};
use audio_processor_traits::simple_processor::MonoAudioProcessor;
use audio_processor_traits::{AudioContext, AudioProcessorSettings};
use generic_handle::GenericHandle;

use crate::state::FilterState;

pub use self::filter::{Filter, FilterType};

/// Raw filter
pub mod filter;
/// `AudioProcessorHandle` implementation, for generic processor handle
pub mod generic_handle;

/// An [`AudioProcessor`] which holds a [`Filter`]. Easy to use DSP filter.
///
/// After setting the filter type with [`FilterProcessor::set_filter_type`], use the filter with the
/// [`AudioProcessor::prepare`] and [`AudioProcessor::process`] methods.
///
/// ```
/// use audio_processor_traits::audio_buffer::{OwnedAudioBuffer, VecAudioBuffer};
/// use audio_processor_traits::{AudioProcessor, BufferProcessor, AudioProcessorSettings};
/// use augmented_dsp_filters::rbj::{FilterProcessor, FilterType};
///
/// let mut audio_buffer = VecAudioBuffer::new();
/// audio_buffer.resize(2, 1 * 44100, 0.0);
/// let settings = AudioProcessorSettings {
///     sample_rate: 44100.0,
///     ..AudioProcessorSettings::default()
/// };
///
/// let mut filter_processor: FilterProcessor<f32> = FilterProcessor::new(FilterType::LowPass);
/// filter_processor.set_cutoff(880.0);
/// filter_processor.set_q(1.0);
///
/// let mut filter_processor = BufferProcessor(filter_processor);
/// filter_processor.prepare(settings);
///
/// filter_processor.process(&mut audio_buffer);
/// ```
pub struct FilterProcessor<
    SampleType: Pow<SampleType, Output = SampleType> + Debug + Float + FloatConst,
> {
    filter_type: FilterType,
    filter: Filter<SampleType>,
    sample_rate: SampleType,
    cutoff: SampleType,
    q: SampleType,
    gain_db: SampleType,
    slope: SampleType,
}

impl Default for FilterProcessor<f32> {
    fn default() -> Self {
        Self::new(FilterType::LowPass)
    }
}

impl<SampleType> AudioProcessorHandleProvider for FilterProcessor<SampleType>
where
    SampleType: Pow<SampleType, Output = SampleType> + Debug + Float + FloatConst,
{
    fn generic_handle(&self) -> AudioProcessorHandleRef {
        make_handle_ref(GenericHandle {})
    }
}

impl<SampleType> FilterProcessor<SampleType>
where
    SampleType: Pow<SampleType, Output = SampleType> + Debug + Float + FloatConst + std::iter::Sum,
{
    /// Create a new [`FilterProcessor`] with the [`FilterType`] and an initial state.
    ///
    /// Sample-rate, cut-off, q, gain and slope will be set to defaults, but should be changed.
    pub fn new(filter_type: FilterType) -> Self {
        Self {
            filter_type,
            filter: Filter::new(),
            sample_rate: SampleType::from(44100.0).unwrap(),
            cutoff: SampleType::from(880.0).unwrap(),
            q: SampleType::from(1.0).unwrap(),
            gain_db: SampleType::from(1.0).unwrap(),
            slope: SampleType::from(0.5).unwrap(),
        }
    }

    /// Change the filter-type
    pub fn set_filter_type(&mut self, filter_type: FilterType) {
        self.filter_type = filter_type;
        self.setup();
    }

    /// Change the cut-off
    pub fn set_cutoff(&mut self, cutoff: SampleType) {
        self.cutoff = cutoff;
        self.setup();
    }

    /// Change the q
    pub fn set_q(&mut self, q: SampleType) {
        self.q = q;
        self.setup();
    }

    /// Change the center-frequency
    pub fn set_center_frequency(&mut self, center_frequency: SampleType) {
        self.cutoff = center_frequency;
        self.setup();
    }

    /// Change the slope
    pub fn set_slope(&mut self, slope: SampleType) {
        self.slope = slope;
        self.setup();
    }

    /// Change the gain
    pub fn set_gain_db(&mut self, gain_db: SampleType) {
        self.gain_db = gain_db;
        self.setup();
    }

    /// Set the sample-rate
    pub fn set_sample_rate(&mut self, sample_rate: SampleType) {
        self.sample_rate = sample_rate;
    }

    /// Set-up the filter for playback
    pub fn setup(&mut self) {
        match self.filter_type {
            FilterType::LowPass => {
                self.filter
                    .setup_low_pass(self.sample_rate, self.cutoff, self.q);
            }
            FilterType::HighPass => {
                self.filter
                    .setup_high_pass(self.sample_rate, self.cutoff, self.q);
            }
            FilterType::BandPass1 => {
                self.filter
                    .setup_band_pass1(self.sample_rate, self.cutoff, self.q);
            }
            FilterType::BandPass2 => {
                self.filter
                    .setup_band_pass2(self.sample_rate, self.cutoff, self.q);
            }
            FilterType::BandStop => {
                self.filter
                    .setup_band_stop(self.sample_rate, self.cutoff, self.q);
            }
            FilterType::LowShelf => {
                self.filter.setup_low_shelf(
                    self.sample_rate,
                    self.cutoff,
                    self.gain_db,
                    self.slope,
                );
            }
            FilterType::HighShelf => {
                self.filter.setup_high_shelf(
                    self.sample_rate,
                    self.cutoff,
                    self.gain_db,
                    self.slope,
                );
            }
        }
    }
}

impl<SampleType> MonoAudioProcessor for FilterProcessor<SampleType>
where
    SampleType: Pow<SampleType, Output = SampleType>
        + Debug
        + Float
        + FloatConst
        + Send
        + Sync
        + std::iter::Sum<SampleType>,
{
    type SampleType = SampleType;

    fn m_prepare(&mut self, context: &mut AudioContext, settings: AudioProcessorSettings) {
        self.sample_rate = SampleType::from(settings.sample_rate()).unwrap();
        self.setup();
    }

    fn m_process(
        &mut self,
        context: &mut AudioContext,
        sample: Self::SampleType,
    ) -> Self::SampleType {
        self.filter.state.process1(
            &self.filter.coefficients,
            sample,
            self.filter.denormal_prevention.alternating_current(),
        )
    }
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::charts::*;

    use audio_processor_traits::simple_processor::BufferProcessor;

    use super::*;

    #[test]
    fn test_band_pass_filter_frequency_response() {
        use FilterType::*;

        let filters = vec![
            ("low-pass", LowPass),
            ("high-pass", HighPass),
            ("band-pass1", BandPass1),
            ("band-pass2", BandPass2),
            ("band-stop", BandStop),
            ("low-shelf", LowShelf),
            ("high-shelf", HighShelf),
        ];

        for (filter_name, filter_type) in filters {
            let mut processor = FilterProcessor::new(filter_type);
            processor.set_cutoff(880.0);
            let mut processor = BufferProcessor(processor);
            generate_frequency_response_plot(
                &format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/src/rbj/mod.rs"),
                &format!("{}-880hz-frequency-response", filter_name),
                &mut processor,
            );
        }
    }
}
