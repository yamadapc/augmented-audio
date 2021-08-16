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
