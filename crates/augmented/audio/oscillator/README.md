# augmented_oscillator


Very simple implementation of an oscillator.

## Examples
### Sine oscillator
```rust
let sample_rate = 44100.0;
let mut osc = augmented_oscillator::Oscillator::sine(sample_rate);
osc.set_frequency(40.0);  // set freq. in Hz
let _sample = osc.next_sample(); // tick the oscillator forward
```

### Wave-table oscillator
```rust
use augmented_oscillator::{Oscillator, wavetable::WaveTableOscillator};

let sample_rate = 44100.0;
// let mut osc = WaveTableOscillator::new(vec![/* your wave table data */]);
// You can either ^^^^ provide your own table (and update it at runtime) or generate a table
// of a certain length (100 sample here) from a function oscillator
let mut osc = WaveTableOscillator::from_oscillator(Oscillator::sine(sample_rate), 100);
osc.set_frequency(40.0);  // set freq. in Hz
let _sample = osc.next_sample(); // tick the oscillator forward
```

### Custom oscillator generator function
```rust
let sample_rate = 44100.0;
let mut osc = augmented_oscillator::Oscillator::new_with_sample_rate(
    sample_rate,
    move |phase: f32| phase.sin()
);
osc.set_frequency(40.0);  // set freq. in Hz
let _sample = osc.next_sample(); // tick the oscillator forward
```

License: MIT
