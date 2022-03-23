use augmented_atomics::AtomicF32;

pub struct LFOHandle {
    pub amount: AtomicF32,
    pub frequency: AtomicF32,
}

impl Default for LFOHandle {
    fn default() -> Self {
        LFOHandle {
            amount: 1.0.into(),
            frequency: 1.0.into(),
        }
    }
}
