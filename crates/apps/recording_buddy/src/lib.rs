#[no_mangle]
pub extern "C" fn initialize_recording_buddy() {
    wisual_logger::init_from_env();
    log::info!("Recording buddy 0.1.0");
}
