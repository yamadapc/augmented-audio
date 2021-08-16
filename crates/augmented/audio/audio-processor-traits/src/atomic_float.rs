use std::sync::atomic::{AtomicU32, Ordering};

/// Simple atomic floating point variable with relaxed ordering.
///
/// Fork of atomic float from rust-vst.
pub struct AtomicF32 {
    atomic: AtomicU32,
}

impl AtomicF32 {
    /// New atomic float with initial value `value`.
    pub fn new(value: f32) -> AtomicF32 {
        AtomicF32 {
            atomic: AtomicU32::new(value.to_bits()),
        }
    }

    /// Get the current value of the atomic float with relaxed ordering.
    pub fn get(&self) -> f32 {
        f32::from_bits(self.atomic.load(Ordering::Relaxed))
    }

    /// Set the value of the atomic float to `value` with relaxed ordering.
    pub fn set(&self, value: f32) {
        self.atomic.store(value.to_bits(), Ordering::Relaxed)
    }

    /// Get the current value of the atomic float with `ordering`.
    pub fn load(&self, ordering: Ordering) -> f32 {
        f32::from_bits(self.atomic.load(ordering))
    }

    /// Set the value of the atomic float to `value` with `ordering`.
    pub fn store(&self, value: f32, ordering: Ordering) {
        self.atomic.store(value.to_bits(), ordering)
    }
}

impl Default for AtomicF32 {
    fn default() -> Self {
        AtomicF32::new(0.0)
    }
}

impl std::fmt::Debug for AtomicF32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.get(), f)
    }
}

impl std::fmt::Display for AtomicF32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.get(), f)
    }
}

impl Clone for AtomicF32 {
    fn clone(&self) -> Self {
        AtomicF32::from(self.get())
    }
}

impl From<f32> for AtomicF32 {
    fn from(value: f32) -> Self {
        AtomicF32::new(value)
    }
}

impl From<AtomicF32> for f32 {
    fn from(value: AtomicF32) -> Self {
        value.get()
    }
}
