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
//! Data-types for representing volume in:
//!
//! * [`Decibels`]
//! * [`Amplitude`]
//!
//! And conversions between the two.
use std::ops::Mul;

#[cfg(not(feature = "f64"))]
pub type Float = f32;

#[cfg(feature = "f64")]
pub type Float = f64;

/// Represents a reference-less dB value.
#[derive(Default, PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct Decibels {
    decibels: Float,
}

impl Decibels {
    /// Constructs [`Decibels`] from a number
    pub fn from_db(decibels: Float) -> Self {
        Decibels { decibels }
    }

    /// Constructs [`Decibels`] from an amplitude and reference
    pub fn from_amplitude(amplitude: Float, reference_amplitude: Float) -> Self {
        Decibels::from_db(amplitude_to_db(amplitude, reference_amplitude))
    }

    /// Changes this [`Decibels`] internal dB value to match `amplitude`
    pub fn set_amplitude(&mut self, amplitude: Float, reference_amplitude: Float) {
        self.decibels = amplitude_to_db(amplitude, reference_amplitude)
    }

    /// Changes this [`Decibels`] internal dB value
    pub fn set_db(&mut self, db: Float) {
        self.decibels = db;
    }

    /// Converts this [`Decibels`] to `amplitude`
    pub fn as_amplitude(&self, reference_amplitude: Float) -> Float {
        db_to_amplitude(self.decibels, reference_amplitude)
    }

    /// Returns this [`Decibels`] inner value
    pub fn as_db(&self) -> Float {
        self.decibels
    }

    /// Converts this [`Decibels`] to amplitude
    pub fn amplitude(&self, reference_amplitude: Float) -> Amplitude {
        Amplitude::from_db(self.decibels, reference_amplitude)
    }
}

impl From<Float> for Decibels {
    fn from(db: Float) -> Self {
        Self::from_db(db)
    }
}

impl From<Decibels> for Float {
    fn from(db: Decibels) -> Self {
        db.as_db()
    }
}

impl Mul for Decibels {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_db(self.decibels * rhs.decibels)
    }
}

/// Represents an amplitude measurement or constant.
#[derive(Default, PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct Amplitude {
    amplitude: Float,
}

impl Amplitude {
    /// Constructs an [`Amplitude`]
    pub fn from_amplitude(amplitude: Float) -> Self {
        Amplitude { amplitude }
    }

    /// Constructs an [`Amplitude`] from a dB measurement and a reference value
    pub fn from_db(db: Float, reference_amplitude: Float) -> Self {
        Amplitude::from_amplitude(db_to_amplitude(db, reference_amplitude))
    }

    /// Changes this [`Amplitude`] inner value
    pub fn set_amplitude(&mut self, amplitude: Float) {
        self.amplitude = amplitude;
    }

    /// Changes this [`Amplitude`] inner value to match dB
    pub fn set_db(&mut self, db: Float, reference_amplitude: Float) {
        self.amplitude = db_to_amplitude(db, reference_amplitude);
    }

    /// Gets this [`Amplitude`] inner value
    pub fn as_amplitude(&self) -> Float {
        self.amplitude
    }

    /// Gets this [`Amplitude`] inner value as dB
    pub fn as_db(&self, reference_amplitude: Float) -> Float {
        amplitude_to_db(self.amplitude, reference_amplitude)
    }

    /// Converts this [`Amplitude`] to [`Decibels`]
    pub fn decibels(&self, reference_amplitude: Float) -> Decibels {
        Decibels::from_amplitude(self.amplitude, reference_amplitude)
    }
}

impl From<Float> for Amplitude {
    fn from(volume: Float) -> Self {
        Self::from_amplitude(volume)
    }
}

impl From<Amplitude> for Float {
    fn from(amp: Amplitude) -> Self {
        amp.as_amplitude()
    }
}

impl Mul for Amplitude {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_amplitude(self.amplitude * rhs.amplitude)
    }
}

impl Mul<Float> for Amplitude {
    type Output = Self;

    fn mul(self, rhs: Float) -> Self::Output {
        Self::from_amplitude(self.amplitude * rhs)
    }
}

/// Convert decibels to amplitude
pub fn db_to_amplitude(db: Float, reference_amplitude: Float) -> Float {
    Float::powf(10.0, db / 20.0) * reference_amplitude
}

/// Convert amplitude to decibels
pub fn amplitude_to_db(volume: Float, reference_amplitude: Float) -> Float {
    20.0 * (volume / reference_amplitude).log10()
}

#[cfg(test)]
mod tests {
    use audio_processor_testing_helpers::assert_f_eq;

    use super::*;

    #[test]
    fn it_can_be_created() {
        let volume = Amplitude::from_amplitude(1.0);
        assert_f_eq!(volume.as_amplitude(), 1.0);
    }

    #[test]
    fn it_can_be_converted_to_db() {
        let volume = Amplitude::from_amplitude(1.0);
        let db = volume.decibels(1.0);
        assert_f_eq!(db.as_db(), 0.0);
    }
}
