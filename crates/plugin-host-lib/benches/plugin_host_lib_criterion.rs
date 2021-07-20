mod cpal_buffer_conversion;
mod rms;

use criterion::{criterion_group, criterion_main};

criterion_group!(
    benches,
    rms::criterion_benchmark,
    cpal_buffer_conversion::criterion_benchmark
);
criterion_main!(benches);
