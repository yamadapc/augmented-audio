use std::time::Instant;

pub fn time<T>(label: &str, body: impl FnOnce() -> T) -> T {
    let start = Instant::now();
    let result = body();
    log::info!("{} duration={}ms", label, start.elapsed().as_millis());
    result
}
