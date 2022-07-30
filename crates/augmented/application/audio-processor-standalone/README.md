# audio-processor-standalone

## Augmented Audio: Audio Processor Standalone
[![crates.io](https://img.shields.io/crates/v/audio-processor-standalone.svg)](https://crates.io/crates/audio-processor-standalone)
[![docs.rs](https://docs.rs/audio-processor-standalone/badge.svg)](https://docs.rs/audio-processor-standalone/)
- - -
This is part of <https://github.com/yamadapc/augmented-audio>. Please review its goals. This
crate builds upon [`audio_processor_traits::AudioProcessor`](https://docs.rs/audio-processor-traits/latest/audio_processor_traits/trait.AudioProcessor.html).

Provides a stand-alone audio-processor runner for [`AudioProcessor`](https://docs.rs/audio-processor-traits/latest/audio_processor_traits/trait.AudioProcessor.html)
implementations.

### Navigating the documentation
* Look at exported functions & macros; the structs/traits are for more advanced/internal usage.
* Start with [`audio_processor_main`] and [`audio_processor_main_with_midi`]
* There are plenty examples in the `augmented-audio` repository

The gist of it is:

1. Implement [`AudioProcessor`](https://docs.rs/audio-processor-traits/latest/audio_processor_traits/trait.AudioProcessor.html)
   or [`SimpleAudioProcessor`](https://docs.rs/audio-processor-traits/latest/audio_processor_traits/trait.SimpleAudioProcessor.html)
   from [`audio_processor_traits`](https://docs.rs/audio-processor-traits)
2. Call `audio_processor_main(processor)`
3. You now have a CLI for rendering online (CPAL, use your mic)  or offline (pass a file through your processor & write
   the results to a `.wav`)

A VST may also be generated through the `standalone_vst` module and by enabling the `vst`
feature flag.

### Example usage

Declare the `AudioProcessor`:

```rust
use audio_processor_traits::{AudioBuffer, AudioProcessor};

struct GainProcessor {}

impl GainProcessor { fn new() -> Self { GainProcessor {} }}

impl AudioProcessor for GainProcessor {
    type SampleType = f32;
    fn process<BufferType: AudioBuffer<SampleType=Self::SampleType>>(&mut self, data: &mut BufferType) {
        for sample in data.slice_mut() {
           *sample = *sample * 0.4;
        }
    }
}
```

Declare the main function:

```rust
fn main() {
    let processor = GainProcessor::new();
    audio_processor_standalone::audio_processor_main(processor);
}
```

### Usage of the command-line
```rust
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

License: MIT
