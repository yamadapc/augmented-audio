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
//! Looper representation of the play-head state. An abstract [`TimeInfo`] provider
//! [`TimeInfoProvider`] wraps both `vst` (hosted mode) and [`PlayHead`] (stand-alone/host mode).
//!
//! * hosted-mode - The DAW provides play-head & tempo
//! * standalone-mode - The [`TimeInfoProviderImpl`] object provides play-head & tempo
use std::sync::atomic::{AtomicBool, Ordering};

use derive_builder::Builder;
use mockall::automock;

#[cfg(not(target_os = "ios"))]
use audio_processor_standalone::standalone_vst::vst;
#[cfg(not(target_os = "ios"))]
pub use audio_processor_standalone::standalone_vst::vst::plugin::HostCallback;
use augmented_atomics::AtomicValue;
use augmented_playhead::{PlayHead, PlayHeadOptions};
use metronome::MetronomePlayhead;

#[cfg(target_os = "ios")]
pub type HostCallback = ();

/// Represents instantaneous play-head state. The current tempo, position and "is playing" state can
/// be queried from this object.
#[derive(Builder)]
pub struct TimeInfo {
    tempo: Option<f64>,
    position_samples: f64,
    position_beats: Option<f64>,
    is_playing: bool,
}

impl TimeInfo {
    /// Current tempo if set
    pub fn tempo(&self) -> Option<f64> {
        self.tempo
    }

    /// Current position as a fraction of samples
    pub fn position_samples(&self) -> f64 {
        self.position_samples
    }

    /// Current position in nº of beats
    pub fn position_beats(&self) -> Option<f64> {
        self.position_beats
    }

    /// Whether the playhead is moving
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }
}

/// Abstract `TimeInfo` provider trait. This is useful for mocking out the playhead on unit-tests.
#[automock]
pub trait TimeInfoProvider {
    /// The current [`TimeInfo`] object
    fn get_time_info(&self) -> TimeInfo;
    /// Must be called for each sample, this moves the playhead
    fn tick(&self) {
        self.tick_n(1)
    }
    /// Alternative for [`TimeInfoProvider::tick`], should move the playhead by `num_samples`
    fn tick_n(&self, num_samples: u32);
    /// Transport control, should start the playhead
    fn play(&self);
    /// Transport control, should stop the playhead (pause & reset to 0)
    fn stop(&self);
    /// Transport control, should pause the playhead
    fn pause(&self);
}

/// Concrete [`TimeInfoProvider`] implementation that supports both a "hosted" mode for VST where
/// the playhead state is queried from the VST Host and a "stand-alone" mode, where the playhead
/// state is managed internally by using [`PlayHead`].
///
/// On `ios` only stand-alone mode is supported as the `vst` crate can't work.
///
/// Transport control methods only work if we're in stand-alone mode. It potentially would be
/// possible to change the host's playhead via the VST APIs, but that has not been implemented or
/// investigated.
pub struct TimeInfoProviderImpl {
    #[allow(unused)] // Prevent iOS warnings
    host_callback: Option<HostCallback>,
    playhead: PlayHead,
    is_playing: AtomicBool,
}

impl TimeInfoProvider for TimeInfoProviderImpl {
    #[cfg(target_os = "ios")]
    fn get_time_info(&self) -> TimeInfo {
        self.get_playhead_time_info()
    }

    #[cfg(not(target_os = "ios"))]
    fn get_time_info(&self) -> TimeInfo {
        let host_time_info = get_host_time_info(self.host_callback.as_ref());

        host_time_info.unwrap_or_else(|| self.get_playhead_time_info())
    }

    fn tick_n(&self, n: u32) {
        if self.is_playing.load(Ordering::Relaxed) {
            self.playhead.accept_samples(n);
        }
    }

    fn play(&self) {
        self.is_playing.store(true, Ordering::Relaxed);
    }

    fn stop(&self) {
        self.is_playing.store(false, Ordering::Relaxed);
        self.playhead.set_position_seconds(0.0);
    }

    fn pause(&self) {
        self.is_playing.store(false, Ordering::Relaxed);
    }
}

impl TimeInfoProviderImpl {
    /// Create provider with given vst [`HostCallback`] or no callback.
    pub fn new(host_callback: Option<HostCallback>) -> Self {
        TimeInfoProviderImpl {
            host_callback,
            playhead: PlayHead::new(PlayHeadOptions::new(None, None, None)),
            is_playing: AtomicBool::new(false),
        }
    }

    pub fn playhead(&self) -> &PlayHead {
        &self.playhead
    }

    pub fn set_tempo(&self, tempo: f32) {
        self.playhead.set_tempo(tempo);
    }

    pub fn set_sample_rate(&self, sample_rate: f32) {
        self.playhead.set_sample_rate(sample_rate);
    }

    /// Same as [`get_host_time_info`] for standalone mode
    fn get_playhead_time_info(&self) -> TimeInfo {
        TimeInfo {
            tempo: self.playhead.options().tempo().map(|t| t as f64),
            position_samples: self.playhead.position_samples() as f64,
            position_beats: self
                .playhead
                .options()
                .tempo()
                .map(|_| self.playhead.position_beats()),
            is_playing: self.is_playing.get(),
        }
    }
}

/// Return the VST API time-info, converted into our object.
///
/// Same as [`TimeInfoProviderImpl::get_playhead_time_info`] but for hosted mode.
#[cfg(not(target_os = "ios"))]
fn get_host_time_info<H: vst::host::Host>(host: Option<&H>) -> Option<TimeInfo> {
    host.as_ref()
        .and_then(|cb| {
            cb.get_time_info(
                (vst::api::TimeInfoFlags::TEMPO_VALID | vst::api::TimeInfoFlags::PPQ_POS_VALID)
                    .bits(),
            )
        })
        .map(|vst_time_info| TimeInfo {
            tempo: Some(vst_time_info.tempo),
            position_samples: vst_time_info.sample_pos,
            position_beats: Some(vst_time_info.ppq_pos),
            is_playing: (vst_time_info.flags & vst::api::TimeInfoFlags::TRANSPORT_PLAYING.bits())
                != 0,
        })
}

/// New-type for metronome compatibility, since metronome defines its own playhead logic.
///
/// This type implements [`MetronomePlayhead`] for a [`Shared`] reference of [`TimeInfoProvidedImpl`]
pub struct TimeInfoMetronomePlayhead(pub audio_garbage_collector::Shared<TimeInfoProviderImpl>);

impl MetronomePlayhead for TimeInfoMetronomePlayhead {
    fn position_beats(&self) -> f64 {
        self.0.playhead().position_beats()
    }
}

#[cfg(test)]
mod test {
    use audio_processor_testing_helpers::assert_f_eq;

    use audio_garbage_collector::make_shared;

    use super::*;

    #[test]
    fn test_time_info_provider_without_tempo_doesnt_move() {
        let time_info_provider = TimeInfoProviderImpl::new(None);
        time_info_provider.play();
        time_info_provider.set_sample_rate(1000.0);
        time_info_provider.tick();
        time_info_provider.tick();
        time_info_provider.tick();
        time_info_provider.tick();
        assert!(time_info_provider
            .get_time_info()
            .position_beats()
            .is_none());
        assert_f_eq!(time_info_provider.get_time_info().position_samples(), 4.0);
    }

    #[test]
    fn test_time_info_provider_stop_will_reset_playhead() {
        let time_info_provider = TimeInfoProviderImpl::new(None);
        time_info_provider.play();
        time_info_provider.set_sample_rate(1000.0);
        time_info_provider.set_tempo(60.0);
        // jump 4 seconds in
        time_info_provider.tick_n(4000);
        let position_beats = time_info_provider.get_time_info().position_beats.unwrap();
        assert_f_eq!(position_beats, 4.0);
        time_info_provider.stop();
        let position_beats = time_info_provider.get_time_info().position_beats.unwrap();
        assert_f_eq!(position_beats, 0.0);
    }

    #[test]
    fn test_time_info_provider_stop_will_prevent_the_playhead_from_updating() {
        let time_info_provider = TimeInfoProviderImpl::new(None);
        time_info_provider.play();
        time_info_provider.set_sample_rate(1000.0);
        time_info_provider.set_tempo(60.0);
        // jump 4 seconds in
        time_info_provider.tick_n(4000);
        let position_beats = time_info_provider.get_time_info().position_beats.unwrap();
        assert_f_eq!(position_beats, 4.0);
        time_info_provider.stop();
        // jump 4 more seconds in
        time_info_provider.tick_n(4000);
        let position_beats = time_info_provider.get_time_info().position_beats.unwrap();
        assert_f_eq!(position_beats, 0.0);
    }

    #[test]
    fn test_time_info_provider_pause_will_prevent_the_playhead_from_updating() {
        let time_info_provider = TimeInfoProviderImpl::new(None);
        time_info_provider.play();
        time_info_provider.set_sample_rate(1000.0);
        time_info_provider.set_tempo(60.0);
        // jump 4 seconds in
        time_info_provider.tick_n(4000);
        let position_beats = time_info_provider.get_time_info().position_beats.unwrap();
        assert_f_eq!(position_beats, 4.0);
        time_info_provider.pause();
        // jump 4 more seconds in
        time_info_provider.tick_n(4000);
        let position_beats = time_info_provider.get_time_info().position_beats.unwrap();
        assert_f_eq!(position_beats, 4.0);
    }

    #[test]
    fn test_time_info_provider_with_tempo_keep_track_of_beats() {
        let time_info_provider = TimeInfoProviderImpl::new(None);
        time_info_provider.play();
        time_info_provider.set_sample_rate(100.0);
        time_info_provider.set_tempo(60.0);

        let time_info = time_info_provider.get_time_info();
        assert!(time_info.position_beats().is_some());
        assert_eq!(time_info.position_beats(), Some(0.0));
        assert_eq!(time_info.position_samples(), 0.0);

        for _i in 0..100 {
            time_info_provider.tick();
        }
        let time_info = time_info_provider.get_time_info();
        assert!(time_info.position_beats().is_some());
        assert_f_eq!(time_info.position_beats().unwrap(), 1.0);
        assert_f_eq!(time_info.position_samples(), 100.0);
    }

    #[test]
    fn test_metronome_playhead() {
        let time_info_provider = TimeInfoProviderImpl::new(None);
        let time_info_provider = make_shared(time_info_provider);
        let metronome_playhead = TimeInfoMetronomePlayhead(time_info_provider.clone());
        assert_f_eq!(metronome_playhead.position_beats(), 0.0);
        time_info_provider.play();
        time_info_provider.set_sample_rate(1000.0);
        time_info_provider.set_tempo(60.0);
        time_info_provider.tick_n(4000);
        assert_f_eq!(metronome_playhead.position_beats(), 4.0);
    }

    #[test]
    fn test_get_host_time_info_when_theres_no_host_returns_none() {
        let result = get_host_time_info::<vst::plugin::HostCallback>(None);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_host_time_info_when_host_returns_none_returns_none() {
        struct MockHost {}

        impl vst::host::Host for MockHost {
            fn get_time_info(&self, _mask: i32) -> Option<vst::api::TimeInfo> {
                None
            }
        }

        let host = MockHost {};
        let result = get_host_time_info(Some(&host));
        assert!(result.is_none());
    }

    #[test]
    fn test_get_host_time_info_converts_vst_time_info() {
        struct MockHost {}

        impl vst::host::Host for MockHost {
            fn get_time_info(&self, _mask: i32) -> Option<vst::api::TimeInfo> {
                Some(vst::api::TimeInfo {
                    ppq_pos: 8.0,
                    tempo: 120.0,
                    sample_pos: 1000.0,
                    flags: vst::api::TimeInfoFlags::TRANSPORT_PLAYING.bits()
                        | vst::api::TimeInfoFlags::TRANSPORT_RECORDING.bits(),
                    ..Default::default()
                })
            }
        }

        let host = MockHost {};
        let result = get_host_time_info(Some(&host));
        assert!(result.is_some());
        let result = result.unwrap();
        assert_f_eq!(result.tempo().unwrap(), 120.0);
        assert_f_eq!(result.position_beats().unwrap(), 8.0);
        assert_f_eq!(result.position_samples(), 1000.0);
        assert_eq!(result.is_playing(), true);
    }
}
