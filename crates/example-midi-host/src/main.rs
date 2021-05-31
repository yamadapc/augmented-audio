mod midi_host;

fn main() {
    wisual_logger::init_from_env();
    let _connection = midi_host::start_midi_host().unwrap();
    std::thread::park();
}
