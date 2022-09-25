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
//! # Augmented Audio: Audio Processor Standalone
//! [![crates.io](https://img.shields.io/crates/v/audio-processor-standalone.svg)](https://crates.io/crates/audio-processor-standalone)
//! [![docs.rs](https://docs.rs/audio-processor-standalone/badge.svg)](https://docs.rs/audio-processor-standalone/)
//! - - -
//! This is part of <https://github.com/yamadapc/augmented-audio>. Please review its goals. This
//! crate builds upon [`audio_processor_traits::AudioProcessor`](https://docs.rs/audio-processor-traits/latest/audio_processor_traits/trait.AudioProcessor.html).
//!
//! Provides a stand-alone audio-processor runner for [`AudioProcessor`](https://docs.rs/audio-processor-traits/latest/audio_processor_traits/trait.AudioProcessor.html)
//! implementations.
//!
//! ## Navigating the documentation
//! * Look at exported functions & macros; the structs/traits are for more advanced/internal usage.
//! * Start with [`audio_processor_main`] and [`audio_processor_main_with_midi`]
//! * There are plenty examples in the `augmented-audio` repository
//!
//! The gist of it is:
//!
//! 1. Implement [`AudioProcessor`](https://docs.rs/audio-processor-traits/latest/audio_processor_traits/trait.AudioProcessor.html)
//!    or [`SimpleAudioProcessor`](https://docs.rs/audio-processor-traits/latest/audio_processor_traits/trait.SimpleAudioProcessor.html)
//!    from [`audio_processor_traits`](https://docs.rs/audio-processor-traits)
//! 2. Call `audio_processor_main(processor)`
//! 3. You now have a CLI for rendering online (CPAL, use your mic)  or offline (pass a file through your processor & write
//!    the results to a `.wav`)
//!
//! A VST may also be generated through the `standalone_vst` module and by enabling the `vst`
//! feature flag.
//!
//! ## Example usage
//!
//! Declare the `AudioProcessor`:
//!
//! ```rust
//! use audio_processor_traits::{AudioBuffer, AudioProcessor};
//!
//! struct GainProcessor {}
//!
//! impl GainProcessor { fn new() -> Self { GainProcessor {} }}
//!
//! impl AudioProcessor for GainProcessor {
//!     type SampleType = f32;
//!     fn process<BufferType: AudioBuffer<SampleType=Self::SampleType>>(&mut self, data: &mut BufferType) {
//!         for sample in data.slice_mut() {
//!            *sample = *sample * 0.4;
//!         }
//!     }
//! }
//! ```
//!
//! Declare the main function:
//!
//! ```ignore
//! fn main() {
//!     let processor = GainProcessor::new();
//!     audio_processor_standalone::audio_processor_main(processor);
//! }
//! ```
//!
//! ## Usage of the command-line
//! ```ignore
//! audio-processor-standalone
//!
//! USAGE:
//! my-crate [OPTIONS]
//!
//! FLAGS:
//! -h, --help       Prints help information
//! -V, --version    Prints version information
//!
//! OPTIONS:
//! -i, --input-file <INPUT_PATH>              An input audio file to process
//! --midi-input-file <MIDI_INPUT_FILE>    If specified, this MIDI file will be passed through the processor
//! -o, --output-file <OUTPUT_PATH>            If specified, will render offline into this file (WAV)
//! ```

use basedrop::Handle;
use cpal::traits::HostTrait;

use crate::standalone_cpal::StandaloneStartOptions;
use crate::standalone_processor::StandaloneOptions;
use audio_processor_traits::{AudioProcessor, MidiEventHandler};
use options::{ParseOptionsParams, RenderingOptions};
#[doc(inline)]
pub use standalone_cpal::{
    audio_processor_start, audio_processor_start_with_midi, standalone_start,
    standalone_start_with, StandaloneHandles,
};
#[doc(inline)]
pub use standalone_processor::{
    StandaloneAudioOnlyProcessor, StandaloneProcessor, StandaloneProcessorImpl,
};

/// Options handling for standalone processor
pub mod options;
/// Online standalone implementation using CPAL
pub mod standalone_cpal;
/// Internal wrapper types
pub mod standalone_processor;

/// Offline rendering implementation (offline functionality will not work on iOS for now)
#[cfg(not(target_os = "ios"))]
pub mod offline;

/// VST support (VST is not compiled for iOS)
#[cfg(not(target_os = "ios"))]
pub mod standalone_vst;

/// A default main function for an [`AudioProcessor`] and [`MidiEventHandler`].
///
/// Run an [`AudioProcessor`] / [`MidiEventHandler`] as a stand-alone cpal app and forward MIDI
/// messages received on all inputs to it. Same as `audio_processor_main`, but requires
/// [`MidiEventHandler`] to support MIDI.
///
/// Will internally create [`cpal::Stream`], [`audio_processor_standalone_midi::MidiHost`] and park the current thread. If the thread
/// is unparked the function will exit and the audio/MIDI threads will stop once these structures
/// are dropped.
pub fn audio_processor_main_with_midi<
    Processor: AudioProcessor<SampleType = f32> + MidiEventHandler + Send + 'static,
>(
    audio_processor: Processor,
    handle: &Handle,
) {
    let app = StandaloneProcessorImpl::new(audio_processor);
    standalone_main(app, Some(handle));
}

/// A default main function for an [`AudioProcessor`].
///
/// Will support running it, based on CLI options, as:
///
/// * CPAL audio app processing audio from default input device and outputting it
/// * CPAL audio app processing an audio input file (MP3)
/// * Offline rendering into a WAV file
///
/// Returns the [`cpal::Stream`] streams. The audio-thread will keep running until these are dropped.
pub fn audio_processor_main<Processor: AudioProcessor<SampleType = f32> + Send + 'static>(
    audio_processor: Processor,
) {
    // Options are parsed twice which isn't good
    let options = options::parse_options(ParseOptionsParams {
        supports_midi: false,
    });
    let app = StandaloneAudioOnlyProcessor::new(
        audio_processor,
        StandaloneOptions {
            input_device: options.rendering().input_device(),
            output_device: options.rendering().output_device(),
            ..Default::default()
        },
    );
    standalone_main(app, None);
}

/// Internal main function used by `audio_processor_main`.
fn standalone_main<SP: StandaloneProcessor>(mut app: SP, handle: Option<&Handle>) {
    let options = options::parse_options(ParseOptionsParams {
        supports_midi: app.supports_midi(),
    });

    if handle_list_commands(&options) {
        return;
    }

    match options.rendering() {
        RenderingOptions::Online { .. } => {
            log::info!("Starting stand-alone online rendering with default IO config");
            let _handles = standalone_start_with::<SP, cpal::Host>(
                app,
                StandaloneStartOptions {
                    handle: handle.cloned(),
                    ..StandaloneStartOptions::default()
                },
            );
            std::thread::park();
        }
        #[cfg(not(target_os = "ios"))]
        RenderingOptions::Offline {
            input_file: input_path,
            output_file: output_path,
        } => {
            let midi_input_file = options.midi().input_file.as_ref().map(|midi_input_file| {
                let file_contents =
                    std::fs::read(midi_input_file).expect("Failed to read input MIDI file");
                let (_, midi_file) =
                    augmented_midi::parse_midi_file::<String, Vec<u8>>(&file_contents)
                        .expect("Failed to parse input MIDI file");
                midi_file
            });

            offline::run_offline_render(offline::OfflineRenderOptions {
                app,
                handle,
                input_path,
                output_path,
                midi_input_file,
            });
        }
        #[cfg(target_os = "ios")]
        _ => {
            log::error!("Offline rendering is unsupported on iOS");
            std::process::exit(1)
        }
    }
}

fn handle_list_commands(options: &options::Options) -> bool {
    if options.list_hosts() {
        for host in cpal::available_hosts() {
            println!("{:?}", host.name());
        }
        return true;
    }

    if options.list_input_devices() || options.list_output_devices() {
        use cpal::traits::DeviceTrait;

        let host = cpal::default_host();
        let devices = if options.list_input_devices() {
            host.input_devices()
        } else {
            host.output_devices()
        };
        for device in devices.expect("Failed to list devices") {
            println!("{:?}", device.name().expect("Failed to get device name"));
        }
        return true;
    }

    false
}
