# audio-processor-standalone
[![crates.io](https://img.shields.io/crates/v/audio-processor-standalone.svg)](https://crates.io/crates/audio-processor-standalone)
[![docs.rs](https://docs.rs/audio-processor-standalone/badge.svg)](https://docs.rs/audio-processor-standalone/)
- - -

Provides a stand-alone audio-processor runner for `AudioProcessor` implementations.

The gist of it is:

1. Implement `AudioProcessor` or `SimpleAudioProcessor` from `audio_processor_traits`
2. Call `audio_processor_main(processor)`
3. You now have a CLI for rendering online (CPAL, use your mic)  or offline (pass a file through your processor & write
   the results to a `.wav`)

```rust
use audio_processor_traits::{AudioBuffer, AudioProcessor};

struct SimpleDelayProcessor {}

impl SimpleDelayProcessor { fn new() -> Self { SimpleDelayProcessor {} }}

impl AudioProcessor for SimpleDelayProcessor { /* omitted for brevity */ }

fn main() {
   let processor = SimpleDelayProcessor::new();
   audio_processor_standalone::audio_processor_main(processor);
}
```

## Usage of the command-line
```
audio-processor-standalone

USAGE:
    my-crate [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input-file <INPUT_PATH>              An input audio file to process
        --midi-input-file <MIDI_INPUT_FILE>    If specified, this MIDI file will be passed through the processor
    -o, --output-file <OUTPUT_PATH>            If specified, will render offline into this file (WAV)
```
