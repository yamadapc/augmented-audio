pub struct LooperEngine {}

impl LooperEngine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn start(&self) {}
}

pub fn initialize_logger() {
    let _ = wisual_logger::try_init_from_env();
}

uniffi_macros::include_scaffolding!("looper");
