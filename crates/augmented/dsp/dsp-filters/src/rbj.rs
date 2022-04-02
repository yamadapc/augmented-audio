//! Filters from <https://shepazu.github.io/Audio-EQ-Cookbook/audio-eq-cookbook.html>
//!
//! Ported from [vinniefalco/DSPFilters](https://github.com/vinniefalco/DSPFilters/)
use std::fmt::Debug;

use audio_processor_traits::parameters::{
    AudioProcessorEmptyHandle, AudioProcessorHandleProvider, AudioProcessorHandleRef,
};
use audio_processor_traits::simple_processor::SimpleAudioProcessor;
use audio_processor_traits::{AudioBuffer, AudioProcessorSettings};
use num::pow::Pow;
use num::traits::FloatConst;
use num::Float;

use crate::coefficients::BiquadCoefficients;
use crate::denormal_prevention;
use crate::state::{DirectFormIState, FilterState};

/// Type of a filter
pub enum FilterType {
    LowPass,
    HighPass,
    BandPass1,
    BandPass2,
    BandStop,
    LowShelf,
    HighShelf,
    // TODO: BandShelf, AllPass,
}

pub fn setup_low_pass<Sample: Float + FloatConst>(
    coefficients: &mut BiquadCoefficients<Sample>,
    sample_rate: Sample,
    cutoff_frequency: Sample,
    q: Sample,
) {
    let one = Sample::from(1.0).unwrap();
    let two = Sample::from(2.0).unwrap();

    let w0: Sample = two * Sample::PI() * cutoff_frequency / sample_rate;
    let cs: Sample = w0.cos();
    let sn: Sample = w0.sin();
    let al: Sample = sn / (two * q);
    let b0: Sample = (one - cs) / two;
    let b1: Sample = one - cs;
    let b2: Sample = (one - cs) / two;
    let a0: Sample = one + al;
    let a1: Sample = -two * cs;
    let a2: Sample = one - al;
    coefficients.set_coefficients(a0, a1, a2, b0, b1, b2);
}

pub fn setup_high_pass<Sample: Float + FloatConst>(
    coefficients: &mut BiquadCoefficients<Sample>,
    sample_rate: Sample,
    cutoff_frequency: Sample,
    q: Sample,
) {
    let one = Sample::from(1.0).unwrap();
    let two = Sample::from(2.0).unwrap();

    let w0: Sample = two * Sample::PI() * cutoff_frequency / sample_rate;
    let cs: Sample = w0.cos();
    let sn: Sample = w0.sin();
    let al: Sample = sn / (two * q);
    let b0: Sample = (one + cs) / two;
    let b1: Sample = -(one + cs);
    let b2: Sample = (one + cs) / two;
    let a0: Sample = one + al;
    let a1: Sample = -two * cs;
    let a2: Sample = one - al;
    coefficients.set_coefficients(a0, a1, a2, b0, b1, b2);
}

pub fn setup_band_pass1<Sample: Float + FloatConst>(
    coefficients: &mut BiquadCoefficients<Sample>,
    sample_rate: Sample,
    center_frequency: Sample,
    band_width: Sample,
) {
    let one = Sample::from(1.0).unwrap();
    let two = Sample::from(2.0).unwrap();

    let w0: Sample = two * Sample::PI() * center_frequency / sample_rate;
    let cs: Sample = w0.cos();
    let sn: Sample = w0.sin();
    let al: Sample = sn / (two * band_width);
    let b0: Sample = band_width * al;
    let b1: Sample = Sample::zero();
    let b2: Sample = -band_width * al;
    let a0: Sample = one + al;
    let a1: Sample = -two * cs;
    let a2: Sample = one - al;
    coefficients.set_coefficients(a0, a1, a2, b0, b1, b2);
}

pub fn setup_band_pass2<Sample: Float + FloatConst>(
    coefficients: &mut BiquadCoefficients<Sample>,
    sample_rate: Sample,
    center_frequency: Sample,
    band_width: Sample,
) {
    let one = Sample::from(1.0).unwrap();
    let two = Sample::from(2.0).unwrap();

    let w0: Sample = two * Sample::PI() * center_frequency / sample_rate;
    let cs: Sample = w0.cos();
    let sn: Sample = w0.sin();
    let al: Sample = sn / (two * band_width);
    let b0: Sample = al;
    let b1: Sample = Sample::zero();
    let b2: Sample = -al;
    let a0: Sample = one + al;
    let a1: Sample = -two * cs;
    let a2: Sample = one - al;
    coefficients.set_coefficients(a0, a1, a2, b0, b1, b2);
}

pub fn setup_band_stop<Sample: Float + FloatConst>(
    coefficients: &mut BiquadCoefficients<Sample>,
    sample_rate: Sample,
    center_frequency: Sample,
    band_width: Sample,
) {
    let one = Sample::from(1.0).unwrap();
    let two = Sample::from(2.0).unwrap();

    let w0: Sample = two * Sample::PI() * center_frequency / sample_rate;
    let cs: Sample = w0.cos();
    let sn: Sample = w0.sin();
    let al: Sample = sn / (two * band_width);
    let b0: Sample = one;
    let b1: Sample = -two * cs;
    let b2: Sample = one;
    let a0: Sample = one + al;
    let a1: Sample = -two * cs;
    let a2: Sample = one - al;
    coefficients.set_coefficients(a0, a1, a2, b0, b1, b2);
}

pub fn setup_low_shelf<Sample: Float + FloatConst + Pow<Sample, Output = Sample>>(
    coefficients: &mut BiquadCoefficients<Sample>,
    sample_rate: Sample,
    cutoff_frequency: Sample,
    gain_db: Sample,
    shelf_slope: Sample,
) {
    let gain = Sample::from(10.0)
        .unwrap()
        .pow(gain_db / Sample::from(40.0).unwrap());
    let one = Sample::from(1.0).unwrap();
    let two = Sample::from(2.0).unwrap();

    let w0 = two * Sample::PI() * cutoff_frequency / sample_rate;
    let cs = w0.cos();
    let sn = w0.sin();
    let al = sn / two * ((gain + one / gain) * (one / shelf_slope - one) + two).sqrt();
    let sq = two * gain.sqrt() * al;
    let b0 = gain * ((gain + one) - (gain - one) * cs + sq);
    let b1 = two * gain * ((gain - one) - (gain + one) * cs);
    let b2 = gain * ((gain + one) - (gain - one) * cs - sq);
    let a0 = (gain + one) + (gain - one) * cs + sq;
    let a1 = -two * ((gain - one) + (gain + one) * cs);
    let a2 = (gain + one) + (gain - one) * cs - sq;

    coefficients.set_coefficients(a0, a1, a2, b0, b1, b2);
}

pub fn setup_high_shelf<Sample: Float + FloatConst + Pow<Sample, Output = Sample>>(
    coefficients: &mut BiquadCoefficients<Sample>,
    sample_rate: Sample,
    cutoff_frequency: Sample,
    gain_db: Sample,
    shelf_slope: Sample,
) {
    let gain = Sample::from(10)
        .unwrap()
        .pow(gain_db / Sample::from(40).unwrap());
    let one = Sample::from(1.0).unwrap();
    let two = Sample::from(2.0).unwrap();

    let w0 = two * Sample::PI() * cutoff_frequency / sample_rate;
    let cs = w0.cos();
    let sn = w0.sin();
    let al = sn / two * ((gain + one / gain) * (one / shelf_slope - one) + two).sqrt();
    let sq = two * gain.sqrt() * al;
    let b0 = gain * ((gain + one) + (gain - one) * cs + sq);
    let b1 = -two * gain * ((gain - one) + (gain + one) * cs);
    let b2 = gain * ((gain + one) + (gain - one) * cs - sq);
    let a0 = (gain + one) - (gain - one) * cs + sq;
    let a1 = two * ((gain - one) - (gain + one) * cs);
    let a2 = (gain + one) - (gain - one) * cs - sq;

    coefficients.set_coefficients(a0, a1, a2, b0, b1, b2);
}

/// Holds the state and coefficients for a filter.
pub struct Filter<Sample: Float> {
    coefficients: BiquadCoefficients<Sample>,
    state: DirectFormIState<Sample>,
    denormal_prevention: denormal_prevention::DenormalPrevention<Sample>,
}

impl<Sample: Float> Filter<Sample> {
    /// Create a new empty filter
    pub fn new() -> Self {
        Filter {
            coefficients: BiquadCoefficients::default(),
            state: DirectFormIState::default(),
            denormal_prevention: denormal_prevention::DenormalPrevention::default(),
        }
    }
}

impl<Sample: Float> Default for Filter<Sample> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Sample: Pow<Sample, Output = Sample> + Debug + Float + FloatConst> Filter<Sample> {
    /// Set-up the filter as low-pass with a certain cut-off and Q
    pub fn setup(&mut self, sample_rate: Sample, cutoff_frequency: Sample, q: Sample) {
        self.setup_low_pass(sample_rate, cutoff_frequency, q);
    }

    /// Set-up the filter as low-pass with a certain cut-off and Q
    pub fn setup_low_pass(&mut self, sample_rate: Sample, cutoff_frequency: Sample, q: Sample) {
        setup_low_pass(&mut self.coefficients, sample_rate, cutoff_frequency, q);
    }

    /// Set-up the filter as high-pass with a certain cut-off and Q
    pub fn setup_high_pass(&mut self, sample_rate: Sample, cutoff_frequency: Sample, q: Sample) {
        setup_high_pass(&mut self.coefficients, sample_rate, cutoff_frequency, q);
    }

    /// Set-up the filter as band-pass with a certain center frequency and band-width
    pub fn setup_band_pass1(
        &mut self,
        sample_rate: Sample,
        center_frequency: Sample,
        band_width: Sample,
    ) {
        setup_band_pass1(
            &mut self.coefficients,
            sample_rate,
            center_frequency,
            band_width,
        );
    }

    /// Set-up the filter as band-pass with a certain center frequency and band-width
    pub fn setup_band_pass2(
        &mut self,
        sample_rate: Sample,
        center_frequency: Sample,
        band_width: Sample,
    ) {
        setup_band_pass2(
            &mut self.coefficients,
            sample_rate,
            center_frequency,
            band_width,
        );
    }

    /// Set-up the filter as band-stop with a certain center frequency and band-width
    pub fn setup_band_stop(
        &mut self,
        sample_rate: Sample,
        center_frequency: Sample,
        band_width: Sample,
    ) {
        setup_band_stop(
            &mut self.coefficients,
            sample_rate,
            center_frequency,
            band_width,
        );
    }

    /// Set-up the filter as low-shelf with a certain cut-off, gain and slope
    pub fn setup_low_shelf(
        &mut self,
        sample_rate: Sample,
        cutoff_frequency: Sample,
        gain_db: Sample,
        shelf_slope: Sample,
    ) {
        setup_low_shelf(
            &mut self.coefficients,
            sample_rate,
            cutoff_frequency,
            gain_db,
            shelf_slope,
        );
    }

    /// Set-up the filter as high-shelf with a certain cut-off, gain and slope
    pub fn setup_high_shelf(
        &mut self,
        sample_rate: Sample,
        cutoff_frequency: Sample,
        gain_db: Sample,
        shelf_slope: Sample,
    ) {
        setup_high_shelf(
            &mut self.coefficients,
            sample_rate,
            cutoff_frequency,
            gain_db,
            shelf_slope,
        );
    }

    /// Process an input [`AudioBuffer`] instance. The [`Filter`] struct is mono (see
    /// [`FilterProcessor`] for multi-channel usage).
    ///
    /// A channel must be provided. The buffer will be modified in-place.
    pub fn process_channel<Buffer: AudioBuffer<SampleType = Sample>>(
        &mut self,
        buffer: &mut Buffer,
        channel_index: usize,
    ) {
        for frame in buffer.frames_mut() {
            let input = frame[channel_index];
            let output = self.state.process1(
                &self.coefficients,
                input,
                self.denormal_prevention.alternating_current(),
            );
            frame[channel_index] = output;
        }
    }
}

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

impl<SampleType> AudioProcessorHandleProvider for FilterProcessor<SampleType>
where
    SampleType: Pow<SampleType, Output = SampleType> + Debug + Float + FloatConst,
{
    fn generic_handle(&self) -> AudioProcessorHandleRef {
        use std::sync::Arc;
        Arc::new(AudioProcessorEmptyHandle)
    }
}

impl<SampleType: Pow<SampleType, Output = SampleType> + Debug + Float + FloatConst>
    FilterProcessor<SampleType>
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

impl<SampleType> SimpleAudioProcessor for FilterProcessor<SampleType>
where
    SampleType: Pow<SampleType, Output = SampleType> + Debug + Float + FloatConst + Send + Sync,
{
    type SampleType = SampleType;

    fn s_prepare(&mut self, settings: AudioProcessorSettings) {
        self.sample_rate = SampleType::from(settings.sample_rate()).unwrap();
        self.setup();
    }

    fn s_process_frame(&mut self, frame: &mut [SampleType]) {
        let input = frame[0];
        let output = self.filter.state.process1(
            &self.filter.coefficients,
            input,
            self.filter.denormal_prevention.alternating_current(),
        );
        for value in frame {
            *value = output;
        }
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
                &*format!("{}{}", env!("CARGO_MANIFEST_DIR"), "/src/rbj.rs"),
                &*format!("{}-880hz-frequency-response", filter_name),
                &mut processor,
            );
        }
    }
}
