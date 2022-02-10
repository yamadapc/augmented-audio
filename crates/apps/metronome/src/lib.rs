use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::time::Duration;

use audio_garbage_collector::{make_shared, Shared};
use audio_processor_traits::{AtomicF32, AudioBuffer, AudioProcessor, AudioProcessorSettings};
use augmented_adsr_envelope::Envelope;
use augmented_oscillator::Oscillator;
use augmented_playhead::{PlayHead, PlayHeadOptions};
pub use bridge_generated::*;

mod api;
mod bridge_generated;
mod processor;
