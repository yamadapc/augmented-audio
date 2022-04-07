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
use basedrop::Shared;

use audio_processor_standalone_osc::{OscMap, OscServer};

use crate::audio::multi_track_looper::parameters::LooperId;
use crate::MultiTrackLooperHandle;

pub fn setup_osc_server(handle: Shared<MultiTrackLooperHandle>) {
    let mut osc_map: OscMap<Shared<MultiTrackLooperHandle>> = OscMap::default();
    osc_map.add(
        "/looper/record",
        Box::new(|handle, _msg| {
            log::info!("Toggle recording");
            handle.start_recording(LooperId(0))
        }),
    );

    osc_map.add(
        "/looper/play",
        Box::new(|handle, _msg| {
            log::info!("Toggle playback");
            handle.toggle_playback(LooperId(0))
        }),
    );

    osc_map.add(
        "/looper/clear",
        Box::new(|handle, _msg| {
            log::info!("Clear");
            handle.clear(LooperId(0));
        }),
    );

    let osc_server = OscServer::new(handle, osc_map);
    let _ = std::thread::Builder::new()
        .name(String::from("looper_osc_server"))
        .spawn(move || {
            if let Err(err) = osc_server.start() {
                log::error!("OscServer has exited with {}", err);
            }
        });
}
