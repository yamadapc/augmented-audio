//! A partial port of [vinniefalco/DSPFilters](https://github.com/vinniefalco/DSPFilters/) to Rust.
//!
//! See [`rbj::FilterProcessor`] for a starting-point.
//!
//! Provides RBJ filters:
//!
//! * [`rbj::FilterType::LowPass`]
//! * [`rbj::FilterType::HighPass`]
//! * [`rbj::FilterType::BandPass1`]
//! * [`rbj::FilterType::BandPass2`]
//! * [`rbj::FilterType::BandStop`]
//! * [`rbj::FilterType::LowShelf`]
//! * [`rbj::FilterType::HighShelf`]

/// RBJ filters
pub mod rbj;

/// Filter coefficient structs for internal or low-level use
pub mod coefficients;
/// Denormal prevention struct
pub mod denormal_prevention;
/// State struct
pub mod state;
