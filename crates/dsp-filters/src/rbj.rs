use std::fmt::Debug;

use audio_processor_traits::{AudioBuffer, AudioProcessor, AudioProcessorSettings};
use num::traits::FloatConst;
use num::Float;

use crate::coefficients::BiquadCoefficients;
use crate::denormal_prevention;
use crate::state::{DirectFormIState, FilterState};
use num::pow::Pow;

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

pub struct Filter<Sample: Float> {
    coefficients: BiquadCoefficients<Sample>,
    state: DirectFormIState<Sample>,
    denormal_prevention: denormal_prevention::DenormalPrevention<Sample>,
}

impl<Sample: Float> Filter<Sample> {
    pub fn new() -> Self {
        Filter {
            coefficients: BiquadCoefficients::default(),
            state: DirectFormIState::default(),
            denormal_prevention: denormal_prevention::DenormalPrevention::default(),
        }
    }
}

impl<Sample: Pow<Sample, Output = Sample> + Debug + Float + FloatConst> Filter<Sample> {
    pub fn setup(&mut self, sample_rate: Sample, cutoff_frequency: Sample, q: Sample) {
        self.setup_low_pass(sample_rate, cutoff_frequency, q);
    }

    pub fn setup_low_pass(&mut self, sample_rate: Sample, cutoff_frequency: Sample, q: Sample) {
        setup_low_pass(&mut self.coefficients, sample_rate, cutoff_frequency, q);
    }

    pub fn setup_high_pass(&mut self, sample_rate: Sample, cutoff_frequency: Sample, q: Sample) {
        setup_high_pass(&mut self.coefficients, sample_rate, cutoff_frequency, q);
    }

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

    pub fn process_channel<Buffer: AudioBuffer<SampleType = Sample>>(
        &mut self,
        buffer: &mut Buffer,
        channel_index: usize,
    ) {
        for sample_index in 0..buffer.num_samples() {
            let input = buffer.get(channel_index, sample_index);
            let output = self.state.process1(
                &self.coefficients,
                *input,
                self.denormal_prevention.alternating_current(),
            );
            buffer.set(channel_index, sample_index, output);
        }
    }
}

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

impl<SampleType: Pow<SampleType, Output = SampleType> + Debug + Float + FloatConst>
    FilterProcessor<SampleType>
{
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

    pub fn set_filter_type(&mut self, filter_type: FilterType) {
        self.filter_type = filter_type;
        self.setup();
    }

    pub fn set_cutoff(&mut self, cutoff: SampleType) {
        self.cutoff = cutoff;
        self.setup();
    }

    pub fn set_q(&mut self, q: SampleType) {
        self.q = q;
        self.setup();
    }

    pub fn set_center_frequency(&mut self, center_frequency: SampleType) {
        self.cutoff = center_frequency;
        self.setup();
    }

    pub fn set_slope(&mut self, slope: SampleType) {
        self.slope = slope;
        self.setup();
    }

    pub fn set_gain_db(&mut self, gain_db: SampleType) {
        self.gain_db = gain_db;
        self.setup();
    }

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

impl<SampleType> AudioProcessor for FilterProcessor<SampleType>
where
    SampleType: Pow<SampleType, Output = SampleType> + Debug + Float + FloatConst + Send + Sync,
{
    type SampleType = SampleType;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.sample_rate = SampleType::from(settings.sample_rate()).unwrap();
        self.filter.setup(self.sample_rate, self.cutoff, self.q);
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        self.filter.process_channel(data, 0);

        // Mono output
        for sample_index in 0..data.num_samples() {
            let left_output = *data.get(0, sample_index);
            for channel_index in 1..data.num_channels() {
                data.set(channel_index, sample_index, left_output);
            }
        }
    }
}
