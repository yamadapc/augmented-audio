# smooth_value
This wraps interpolation over a value.

```rust
use std::time::Duration;

fn example() {
    let sample_rate = 44100.0;
    let initial_value = 440.0;
    let smoothing_duration = Duration::of_secs(1);
    // set an initial state, sample rate & interpolation duration
    let value = smooth_value::InterpolatedValue::new(sample_rate, smoothing_duration, initial_value);
    // set a target
    value.set(880.0);

    // do this for each sample...
    let _freq = value.next();
    // it'll take 1s for the value to reach the target
}
```