use num::traits::FloatConst;
use num::Float;

use crate::filter::rbj::coefficients::BiquadCoefficients;
use crate::filter::rbj::state::FilterState;
use audio_processor_traits::AudioBuffer;
use std::fmt::Debug;

mod coefficients {
    use num::Float;

    pub struct BiquadCoefficients<Sample: Float> {
        pub(crate) a0: Sample,
        pub(crate) a1: Sample,
        pub(crate) a2: Sample,
        pub(crate) b1: Sample,
        pub(crate) b2: Sample,
        pub(crate) b0: Sample,
    }

    impl<Sample: Float> Default for BiquadCoefficients<Sample> {
        fn default() -> Self {
            BiquadCoefficients {
                a0: Sample::zero(),
                a1: Sample::zero(),
                a2: Sample::zero(),
                b1: Sample::zero(),
                b2: Sample::zero(),
                b0: Sample::zero(),
            }
        }
    }

    impl<Sample: Float> BiquadCoefficients<Sample> {
        pub fn set_coefficients(
            &mut self,
            a0: Sample,
            a1: Sample,
            a2: Sample,
            b0: Sample,
            b1: Sample,
            b2: Sample,
        ) {
            assert!(
                !a0.is_nan()
                    && !a1.is_nan()
                    && !a2.is_nan()
                    && !a0.is_nan()
                    && !b0.is_nan()
                    && !b1.is_nan()
                    && !b2.is_nan()
            );
            self.a0 = a0;
            self.a1 = a1 / a0;
            self.a2 = a2 / a0;
            self.b0 = b0 / a0;
            self.b1 = b1 / a0;
            self.b2 = b2 / a0;

            // log::info!("a0: {}", self.a0.to_f32().unwrap());
            // log::info!("a1: {}", self.a1.to_f32().unwrap());
            // log::info!("a2: {}", self.a2.to_f32().unwrap());
            assert!(
                !self.a0.is_nan()
                    && !self.a1.is_nan()
                    && !self.a2.is_nan()
                    && !self.a0.is_nan()
                    && !self.b0.is_nan()
                    && !self.b1.is_nan()
                    && !self.b2.is_nan()
            );
        }
    }
}

mod denormal_prevention {
    use num::Float;

    static VERY_SMALL_AMOUNT: f64 = 1e-8;

    /// Hack to prevent denormals
    pub struct DenormalPrevention<Sample: Float> {
        value: Sample,
    }

    impl<Sample: Float> Default for DenormalPrevention<Sample> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<Sample: Float> DenormalPrevention<Sample> {
        pub fn new() -> Self {
            DenormalPrevention {
                value: Sample::from(VERY_SMALL_AMOUNT).unwrap(),
            }
        }
    }

    impl<Sample: Float> DenormalPrevention<Sample> {
        #[inline]
        pub fn alternating_current(&mut self) -> Sample {
            self.value = -self.value;
            self.value
        }

        #[inline]
        pub fn direct_current(&mut self) -> Sample {
            Sample::from(VERY_SMALL_AMOUNT).unwrap()
        }
    }
}

mod state {
    use num::Float;

    use crate::filter::rbj::coefficients::BiquadCoefficients;

    pub trait FilterState {
        type Sample: Float;

        fn reset(&mut self);
        fn process1(
            &mut self,
            coefficients: &BiquadCoefficients<Self::Sample>,
            input: Self::Sample,
            very_small_amount: Self::Sample,
        ) -> Self::Sample;
    }

    /// State for applying a second order section to a sample using Direct Form I
    /// Difference equation:
    ///
    /// ```
    /// y[n] = (b0/a0)*x[n] + (b1/a0)*x[n-1] + (b2/a0)*x[n-2]
    ///                     - (a1/a0)*y[n-1] - (a2/a0)*y[n-2]
    /// ```
    pub struct DirectFormIState<Sample: Float> {
        x2: Sample, // x[n - 2]
        y2: Sample, // y[n - 2]
        x1: Sample, // x[n - 1]
        y1: Sample, // y[n - 1]
    }

    impl<Sample: Float> Default for DirectFormIState<Sample> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<Sample: Float> DirectFormIState<Sample> {
        pub fn new() -> Self {
            DirectFormIState {
                x2: Sample::zero(),
                y2: Sample::zero(),
                x1: Sample::zero(),
                y1: Sample::zero(),
            }
        }
    }

    impl<Sample: Float> FilterState for DirectFormIState<Sample> {
        type Sample = Sample;

        fn reset(&mut self) {
            self.x1 = Sample::zero();
            self.x2 = Sample::zero();
            self.y1 = Sample::zero();
            self.y2 = Sample::zero();
        }

        fn process1(
            &mut self,
            coefficients: &BiquadCoefficients<Sample>,
            input: Sample,
            very_small_amount: Sample,
        ) -> Sample {
            let output = {
                let BiquadCoefficients {
                    b0, b1, b2, a1, a2, ..
                } = *coefficients;
                let DirectFormIState { x1, x2, y2, y1 } = *self;
                // log::info!("a1: {:?}", a1.to_f32().unwrap());
                // log::info!("a2: {:?}", a2.to_f32().unwrap());
                // log::info!("b0: {:?}", b0.to_f32().unwrap());
                // log::info!("b1: {:?}", b1.to_f32().unwrap());
                // log::info!("b2: {:?}", b2.to_f32().unwrap());
                // log::info!("x1: {:?}", x1.to_f32().unwrap());
                // log::info!("x2: {:?}", x2.to_f32().unwrap());
                // log::info!("y1: {:?}", y1.to_f32().unwrap());
                // log::info!("y2: {:?}", y2.to_f32().unwrap());
                b0 * input + b1 * x1 + b2 * x2 - a1 * y1 - a2 * y2 + very_small_amount
            };
            self.x2 = self.x1;
            self.y2 = self.y1;
            self.x1 = input;
            self.y1 = output;
            assert!(!self.y1.is_nan());
            assert!(!self.y2.is_nan());

            output
        }
    }

    /// State for applying a second order section to a sample using Direct Form II
    ///
    /// Difference equation:
    ///
    /// ```
    /// v[n] =         x[n] - (a1/a0)*v[n-1] - (a2/a0)*v[n-2]
    /// y(n) = (b0/a0)*v[n] + (b1/a0)*v[n-1] + (b2/a0)*v[n-2]
    /// ```
    pub struct DirectFormIIState<Sample: Float> {
        v1: Sample, // v[-1]
        v2: Sample, // v[-2]
    }

    impl<Sample: Float> FilterState for DirectFormIIState<Sample> {
        type Sample = Sample;

        fn reset(&mut self) {
            self.v1 = Sample::zero();
            self.v2 = Sample::zero();
        }

        fn process1(
            &mut self,
            coefficients: &BiquadCoefficients<Sample>,
            input: Sample,
            very_small_amount: Sample,
        ) -> Sample {
            let BiquadCoefficients {
                a1, a2, b0, b1, b2, ..
            } = *coefficients;
            let DirectFormIIState { v1, v2 } = *self;
            let w = input - a1 * v1 - a2 * v2 + very_small_amount;
            let output = b0 * w + b1 * v1 + b2 * v2;

            self.v2 = v1;
            self.v1 = w;

            output
        }
    }
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
    log::warn!("Calculating low-pass coeffs.");
    coefficients.set_coefficients(a0, a1, a2, b0, b1, b2);
}

pub struct LowPassFilter<Sample: Float> {
    coefficients: BiquadCoefficients<Sample>,
    state: state::DirectFormIState<Sample>,
    denormal_prevention: denormal_prevention::DenormalPrevention<Sample>,
}

impl<Sample: Float> LowPassFilter<Sample> {
    pub fn new() -> Self {
        LowPassFilter {
            coefficients: BiquadCoefficients::default(),
            state: state::DirectFormIState::default(),
            denormal_prevention: denormal_prevention::DenormalPrevention::default(),
        }
    }
}

impl<Sample: Debug + Float + FloatConst> LowPassFilter<Sample> {
    pub fn setup(&mut self, sample_rate: Sample, cutoff_frequency: Sample, q: Sample) {
        setup_low_pass(&mut self.coefficients, sample_rate, cutoff_frequency, q);
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
