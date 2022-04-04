use std::sync::atomic::{AtomicBool, Ordering};

use crate::AtomicValue;

pub struct AtomicOption<T: AtomicValue + Default> {
    is_present: AtomicBool,
    value: T,
}

impl<T: AtomicValue + Default> AtomicOption<T> {
    pub fn new(value: T) -> Self {
        Self {
            is_present: AtomicBool::new(true),
            value,
        }
    }

    pub fn empty() -> Self {
        Self {
            is_present: AtomicBool::new(false),
            value: Default::default(),
        }
    }

    #[inline]
    pub fn set(&self, value: Option<T::Inner>) {
        if let Some(value) = value {
            self.value.set(value);
            self.is_present.store(true, Ordering::Relaxed);
        } else {
            self.is_present.store(false, Ordering::Relaxed);
        }
    }

    #[inline]
    pub fn inner(&self) -> Option<T::Inner> {
        let is_present = self.is_present.load(Ordering::Relaxed);
        if is_present {
            Some(self.value.get())
        } else {
            None
        }
    }
}

impl<T: AtomicValue + Default + From<T::Inner>> From<Option<T::Inner>> for AtomicOption<T> {
    fn from(value: Option<T::Inner>) -> Self {
        if let Some(value) = value {
            Self::new(value.into())
        } else {
            Self::empty()
        }
    }
}
