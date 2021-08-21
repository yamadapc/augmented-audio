use iced_baseview::{IcedWindow, Settings};

use augmented::gui::baseview::{Size, WindowOpenOptions, WindowScalePolicy};
use looper_processor::LooperProcessor;

use crate::ui::Flags;
use crate::ui::LooperApplication;

mod ui;

fn main() {
    augmented::ops::wisual_logger::init_from_env();

    let gc = augmented::audio::gc::GarbageCollector::default();
    let loopi_processor = LooperProcessor::new(gc.handle());
    let processor_handle = loopi_processor.handle();
    let _audio_handles =
        augmented::application::audio_processor_start_with_midi(loopi_processor, gc.handle());

    IcedWindow::<LooperApplication>::open_blocking(Settings {
        window: WindowOpenOptions {
            title: "Looper".to_string(),
            size: Size {
                width: 500.0,
                height: 500.0,
            },
            scale: WindowScalePolicy::SystemScaleFactor,
        },
        flags: Flags { processor_handle },
    });
}
