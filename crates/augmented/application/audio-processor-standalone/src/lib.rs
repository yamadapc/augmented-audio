use basedrop::Handle;

use audio_processor_traits::{AudioProcessor, MidiEventHandler};
use options::Options;
pub use standalone_cpal::audio_processor_start;
pub use standalone_cpal::audio_processor_start_with_midi;
pub use standalone_cpal::standalone_start;
pub use standalone_cpal::StandaloneHandles;
pub use standalone_processor::StandaloneAudioOnlyProcessor;
pub use standalone_processor::StandaloneProcessor;
pub use standalone_processor::StandaloneProcessorImpl;

pub mod offline;
mod options;
mod standalone_cpal;
mod standalone_processor;

/// Run an [`AudioProcessor`] / [`MidiEventHandler`] as a stand-alone cpal app and forward MIDI
/// messages received on all inputs to it.
///
/// Will internally create [`cpal::Stream`], [`MidiHost`] and park the current thread. If the thread
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

/// Run an [`AudioProcessor`] stand-alone cpal app.
///
/// Returns the [`cpal::Stream`] streams. The audio-thread will keep running until these are dropped.
pub fn audio_processor_main<Processor: AudioProcessor<SampleType = f32> + Send + 'static>(
    audio_processor: Processor,
) {
    let app = StandaloneAudioOnlyProcessor::new(audio_processor);
    standalone_main(app, None);
}

fn standalone_main(app: impl StandaloneProcessor, handle: Option<&Handle>) {
    let options = options::parse_options();
    match options {
        Options::OnlineRendering { .. } => {
            log::info!("Starting stand-alone online rendering with default IO config");
            let _handles = standalone_start(app, handle);
            std::thread::park();
        }
        Options::OfflineRendering {
            input_path,
            output_path,
        } => {
            offline::run_offline_render(app, handle, input_path, output_path);
        }
    }
}
