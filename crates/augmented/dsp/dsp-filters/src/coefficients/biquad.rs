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
