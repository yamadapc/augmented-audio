# augmented-dsp-filters

Mechanical port of Vinnie Falco's https://github.com/vinniefalco/DSPFilters/.

Only RBJ filters are ported over. No introspection is supported & the implementation is quite a different (as Rust would
prefer composition to multiple inheritance).

Very untested, be careful with your speakers.

Depends on `audio-processor-traits`. Exports `FilterProcessor` which may be used for general filtering needs.

See `synth` on this repository for a nice working example.

## Low-pass filter example

```rust
use audio_processor_traits::{
    AudioBuffer, AudioProcessor, AudioProcessorSettings
};
use augmented_dsp_filters::rbj::{FilterProcessor, FilterType};

pub struct YourProcessor {
    filter: FilterProcessor<f32>, // <- f32 may be f64 if you wish
}

impl YourProcessor {
    fn new() -> Self {
        Self {
            filter: FilterProcessor::new(FilterType::LowPass),
        }
    }
    
    fn set_cutoff(&mut self, midi_value_between_0_and_127: f32) {
        let cutoff_ratio = midi_value_between_0_and_127 / 127.0;
        let cutoff_freq_hz = 22000.0 * cutoff_ratio;
        self.filter.set_cutoff(cutoff_freq_hz);
    }
    
    fn set_q(&mut self, q: f32) {
        self.filter.set_q(q);
    }
}

impl AudioProcessor for YourProcessor {
    type SampleType = f32;

    fn prepare(&mut self, settings: AudioProcessorSettings) {
        self.filter.prepare(settings);
    }

    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    ) {
        self.filter.process(data);
    }
}
```

## Multi-threading
The filter mutate functions recalculate coefficients for the filter. This should run on the audio-thread only.

In order to integrate with MIDI, see `synth`; this won't be a problem as `AudioProcessor` (stand-alone) will receive
MIDI on the audio-thread. 

For integrating with a GUI thread, the best would probably be to have the audio-thread read the parameters from an
atomic store & update the filter when they change (see `audio-parameter-store` in this repository).

## License
MIT licensed as the original.