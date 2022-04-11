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
//! This crate implements a multi-track, single-track and shuffling-loopers.
//!
//! Multi-track loopers support sequencing, parameter locking, LFOs and effects. The multi-track
//! looper is a sampler/live-looper with a C API. This is integrated onto the Continuous Looper.
//!
//! The multi-track looper uses the following processor graph:
//!
//! [![](https://mermaid.ink/img/pako:eNptkkFrwyAYhv-KfOcm0BxzGBSadd062q07bMQeJJpF1mgwOhiS_z6DTUxCPYg-76s8oBYKSRmk8K1IU6GPLRbIjY3di8bozu-m8yaKHrL8IGXDFFpfPEQO7vIT10UVtRUv9SJ7zDPxy67uzAw_5VlZskK3M_ppd4QLtO4C2tuj0d7nZtHT10EjuaDQfVl4JJOrn4PHFB9Gjyn98h7JfY_R4jhYxHE89TgtPPo4pG_BZB68jy5zfvY2Dt7zgRXUTNWEU_eUti9g0BWrGYbULSlRPxiw6HumoUSzjHItFaQlubZsBcRoef4TBaRaGTaUtpy4b1HfWt0_t6CkKQ)](https://mermaid.live/edit#pako:eNptkkFrwyAYhv-KfOcm0BxzGBSadd062q07bMQeJJpF1mgwOhiS_z6DTUxCPYg-76s8oBYKSRmk8K1IU6GPLRbIjY3di8bozu-m8yaKHrL8IGXDFFpfPEQO7vIT10UVtRUv9SJ7zDPxy67uzAw_5VlZskK3M_ppd4QLtO4C2tuj0d7nZtHT10EjuaDQfVl4JJOrn4PHFB9Gjyn98h7JfY_R4jhYxHE89TgtPPo4pG_BZB68jy5zfvY2Dt7zgRXUTNWEU_eUti9g0BWrGYbULSlRPxiw6HumoUSzjHItFaQlubZsBcRoef4TBaRaGTaUtpy4b1HfWt0_t6CkKQ)
//!
//! Internals:
//!
//! * [`audio`] - Contains most of the audio-thread bits
//!   - [`audio::processor`] - Is a looper
//!   - [`audio::shuffler`] - Is a shuffling looper
//!   - [`audio::loop_quantization`] - Quantization modes & logic
//!   - [`audio::multi_track_looper`] - Continuous Looper processor
//!   - [`audio::midi_map`] - MIDI map store (used in Continuous)
//! * [`services`] - Some IO support for Continuous
//! * [`c_api`] - C API
pub use atomic_refcell::AtomicRefCell;

pub use self::audio::multi_track_looper::parameters;
pub use self::audio::multi_track_looper::parameters::EnvelopeParameter;
pub use self::audio::multi_track_looper::parameters::LooperId;
pub use self::audio::multi_track_looper::{MultiTrackLooper, MultiTrackLooperHandle};
pub use self::audio::processor::handle::LooperHandle as LooperProcessorHandle;
pub use self::audio::processor::handle::LooperHandleThread;
pub use self::audio::processor::handle::LooperOptions;
pub use self::audio::processor::handle::QuantizeMode;
pub use self::audio::processor::handle::QuantizeOptions;
pub use self::audio::processor::LooperProcessor;
pub use self::audio::shuffler::LoopShufflerParams;
pub use self::audio::shuffler::LoopShufflerProcessorHandle;
pub use self::audio::time_info_provider::{TimeInfo, TimeInfoProvider, TimeInfoProviderImpl};
pub use self::c_api::*;
pub use self::services::osc_server::setup_osc_server;

pub mod audio;
#[allow(clippy::missing_safety_doc)]
pub mod c_api;
pub mod engine;
pub mod services;

mod controllers;

mod common;
#[cfg(target_os = "macos")]
#[cfg(test)]
pub mod integration_test;

const MAX_LOOP_LENGTH_SECS: f32 = 600.0;
