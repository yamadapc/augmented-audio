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
use audio_processor_traits::AudioBuffer;
use num::{pow::Pow, traits::FloatConst, Float};
use std::fmt::Debug;

use crate::coefficients::BiquadCoefficients;
use crate::denormal_prevention;
use crate::state::{DirectFormIState, FilterState};

/// Type of a filter
#[derive(Clone, Copy)]
pub enum FilterType {
    LowPass,
    HighPass,
    BandPass1,
    BandPass2,
    BandStop,
    LowShelf,
    HighShelf,
    AllPass,
    BandShelf,
}

/// Calculate low-pass coefficients
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

/// Calculate high-pass coefficients
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

/// Calculate band-pass coefficients
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

/// Calculate band-pass coefficients
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

/// Calculate band-stop coefficients
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

/// Calculate low-shelf coefficients
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

/// Calculate high-shelf coefficients
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

/// Calculate all-pass coefficients
pub fn setup_all_pass<Sample: Float + FloatConst>(
    coefficients: &mut BiquadCoefficients<Sample>,
    sample_rate: Sample,
    center_frequency: Sample,
    q: Sample,
) {
    let one: Sample = Sample::from(1.0).unwrap();
    let two: Sample = Sample::from(2.0).unwrap();

    let w0: Sample = two * Sample::PI() * center_frequency / sample_rate;
    let cs: Sample = w0.cos();
    let sn: Sample = w0.sin();
    let al: Sample = sn / (two * q);
    let b0: Sample = one - al;
    let b1: Sample = -two * cs;
    let b2: Sample = one + al;
    let a0: Sample = one + al;
    let a1: Sample = -two * cs;
    let a2: Sample = one - al;

    coefficients.set_coefficients(a0, a1, a2, b0, b1, b2);
}

// Calculate band-shelf coefficients
pub fn setup_band_shelf<Sample: Float + FloatConst + Pow<Sample, Output = Sample>>(
    coefficients: &mut BiquadCoefficients<Sample>,
    sample_rate: Sample,
    center_frequency: Sample,
    gain_db: Sample,
    band_width: Sample,
) {
    let gain: Sample = Sample::from(10.0)
        .unwrap()
        .pow(gain_db / Sample::from(40.0).unwrap());
    let one: Sample = Sample::from(1.0).unwrap();
    let two: Sample = Sample::from(2.0).unwrap();

    let w0: Sample = two * Sample::PI() * center_frequency / sample_rate;
    let cs: Sample = w0.cos();
    let sn: Sample = w0.sin();
    let al: Sample = sn
        * Sample::from(two.ln() * band_width * w0 / (two * sn))
            .unwrap()
            .sinh();
    let b0: Sample = one + al * gain;
    let b1: Sample = -two * cs;
    let b2: Sample = one - al * gain;
    let a0: Sample = one + al / gain;
    let a1: Sample = -two * cs;
    let a2: Sample = one - al / gain;

    coefficients.set_coefficients(a0, a1, a2, b0, b1, b2);
}

/// Holds the state and coefficients for a filter.
pub struct Filter<Sample: Float> {
    pub coefficients: BiquadCoefficients<Sample>,
    pub state: DirectFormIState<Sample>,
    pub denormal_prevention: denormal_prevention::DenormalPrevention<Sample>,
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

    /// Set-up the filter as all-pass with a certain center-frequency and Q
    pub fn setup_all_pass(&mut self, sample_rate: Sample, center_frequency: Sample, q: Sample) {
        setup_all_pass(&mut self.coefficients, sample_rate, center_frequency, q);
    }

    /// Setup the filter as a band-shelf with a center frequency, gain and band-width
    pub fn setup_band_shelf(
        &mut self,
        sample_rate: Sample,
        center_frequency: Sample,
        gain_db: Sample,
        band_width: Sample,
    ) {
        setup_band_shelf(
            &mut self.coefficients,
            sample_rate,
            center_frequency,
            gain_db,
            band_width,
        );
    }

    /// Process an input [`AudioBuffer`] instance. The [`Filter`] struct is mono (see
    /// [`FilterProcessor`] for multi-channel usage).
    ///
    /// A channel must be provided. The buffer will be modified in-place.
    pub fn process_channel(&mut self, buffer: &mut AudioBuffer<Sample>, channel_index: usize) {
        for sample in buffer.channel_mut(channel_index) {
            let output = self.state.process1(
                &self.coefficients,
                *sample,
                self.denormal_prevention.alternating_current(),
            );
            *sample = output;
        }
    }
}
