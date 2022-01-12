# ADSR Envelope
[![crates.io](https://img.shields.io/crates/v/adsr-envelope.svg)](https://crates.io/crates/adsr-envelope)
[![docs.rs](https://docs.rs/adsr-envelope/badge.svg)](https://docs.rs/adsr-envelope/)
- - -
Implementation of a ADSR envelope.

## Usage
```rust
use std::time::Duration;

use adsr_envelope::Envelope;

fn main() {
  let mut envelope = Envelope::exp();
  envelope.set_sample_rate(44100.0);
  
  envelope.set_attack(Duration::from_millis(200));

  envelope.note_on();
  for i in 10000 {
    let volume = envelope.volume();
    println!("{}", volume);
    envelope.tick();
  }
}
```

## License
MIT
