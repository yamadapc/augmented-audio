# oscillator
Very simple implementation of an oscillator.

```rust
fn example() {
    let sample_rate = 44100.0;
    let osc = oscillator::Oscillator::new_with_sample_rate(
        sample_rate,
        move |phase: f32| phase.sin()
    );
    osc.set_frequency(40.0);  // set freq. in Hz
    let _sample = osc.next(); // tick the oscillator forward
}
```