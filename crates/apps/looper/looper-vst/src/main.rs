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

use iced_baseview::settings::IcedBaseviewSettings;
use iced_baseview::Settings;

use augmented::gui::baseview::{Size, WindowOpenOptions, WindowScalePolicy};
use looper_processor::{LooperOptions, MultiTrackLooper};

use crate::ui::Flags;
use crate::ui::LooperApplication;

mod ui;

fn main() {
    augmented::ops::wisual_logger::init_from_env();

    let loopi_processor = MultiTrackLooper::new(
        LooperOptions {
            ..LooperOptions::default()
        },
        1,
    );
    let processor_handle = loopi_processor.handle().clone();
    let _audio_handles = augmented::application::audio_processor_start_with_midi(
        loopi_processor,
        audio_garbage_collector::handle(),
    );

    iced_baseview::open_blocking::<LooperApplication>(Settings {
        window: WindowOpenOptions {
            title: "Looper".to_string(),
            size: Size {
                width: 700.0,
                height: 700.0,
            },
            scale: WindowScalePolicy::SystemScaleFactor,
            #[cfg(feature = "glow")]
            gl_config: None,
        },
        flags: Flags { processor_handle },
        iced_baseview: IcedBaseviewSettings {
            always_redraw: true,
            ignore_non_modifier_keys: false,
        },
    });
}
