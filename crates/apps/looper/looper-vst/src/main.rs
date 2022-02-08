use iced_baseview::{IcedWindow, Settings};

use augmented::gui::baseview::{Size, WindowOpenOptions, WindowScalePolicy};
use looper_processor::{LooperOptions, LooperProcessor};

use crate::ui::Flags;
use crate::ui::LooperApplication;

mod ui;

fn main() {
    augmented::ops::wisual_logger::init_from_env();

    let loopi_processor = LooperProcessor::new(LooperOptions {
        ..LooperOptions::default()
    });
    let processor_handle = loopi_processor.handle().clone();
    let sequencer_handle = loopi_processor.sequencer_handle().clone();
    let _audio_handles = augmented::application::audio_processor_start_with_midi(
        loopi_processor,
        audio_garbage_collector::handle(),
    );

    IcedWindow::<LooperApplication>::open_blocking(Settings {
        window: WindowOpenOptions {
            title: "Looper".to_string(),
            size: Size {
                width: 700.0,
                height: 300.0,
            },
            scale: WindowScalePolicy::SystemScaleFactor,
        },
        flags: Flags {
            processor_handle,
            sequencer_handle,
            host_callback: None,
        },
    });
}
