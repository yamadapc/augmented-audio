use augmented_audio_gui_basics::multitouch::get_multitouch_devices;
use augmented_audio_gui_basics::prelude::*;
use std::time::Duration;

fn main() {
    let devices = get_multitouch_devices();
    for mut device in devices {
        device.register_contact_frame_callback(|evt, finger, _, _| {
            println!("{:?}", finger.len());
        });
    }
    std::thread::sleep(Duration::from_secs(600));
    // sketch(|ctx| {
    //     let size = ctx.size();
    //     let canvas = ctx.canvas();
    //     canvas.clear(black());
    //
    //     let widget = Rectangle::default();
    //     render(canvas, size, widget.into());
    // })
}
