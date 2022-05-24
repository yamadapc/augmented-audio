# augmented-adsr-envelope

[![crates.io](https://img.shields.io/crates/v/adsr-envelope.svg)](https://crates.io/crates/adsr-envelope)
[![docs.rs](https://docs.rs/adsr-envelope/badge.svg)](https://docs.rs/adsr-envelope/)
- - -
Implementation of an ADSR envelope.

## Basic usage

```rust
use std::time::Duration;

use augmented_adsr_envelope::Envelope;

// Create an exponential envelope.
// The envelope configuration uses atomics, so it doesn't need
// to be an immutable reference.
let envelope = Envelope::exp();

// Set settings
envelope.set_sample_rate(1000.0);
envelope.set_attack(Duration::from_millis(200));

// Trigger the envelope
envelope.note_on();
for i in 0..10000 {
  // Tick the envelope by 1 sample
  envelope.tick();
  // Get the current volume
  let _volume = envelope.volume();
}
// Trigger the release stage
envelope.note_off();
```

## Plots
`Envelope::default();`
------------------
| Attack  | 0.3  |
| Decay   | 0.3  |
| Sustain | 0.8  |
| Release | 0.3  |
------------------
![](https://raw.githubusercontent.com/yamadapc/augmented-audio/master/crates/augmented/audio/adsr-envelope/src/__plots__/default-envelope.png)

`Envelope::exp();`
------------------
| Attack  | 0.3  |
| Decay   | 0.3  |
| Sustain | 0.8  |
| Release | 0.3  |
------------------
![](https://raw.githubusercontent.com/yamadapc/augmented-audio/master/crates/augmented/audio/adsr-envelope/src/__plots__/exp-envelope.png)

License: MIT
