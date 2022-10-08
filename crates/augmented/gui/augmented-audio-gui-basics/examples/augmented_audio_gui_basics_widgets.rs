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
// use augmented_audio_gui_basics::multitouch::get_multitouch_devices;
use augmented_audio_gui_basics::prelude::*;
// use std::time::Duration;

fn main() {
    // let devices = get_multitouch_devices();
    // for mut device in devices {
    //     device.register_contact_frame_callback(|_evt, finger, _, _| {
    //         println!("{:?}", finger.len());
    //     });
    // }
    // std::thread::sleep(Duration::from_secs(600));
    sketch(|ctx| {
        let size = ctx.size();
        let canvas = ctx.canvas();
        canvas.clear(black());

        let widget = Rectangle::default();
        render(canvas, size, widget.into());
    })
}
