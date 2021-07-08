# dsp-filters

Mechanical port of Vinnie Falco's https://github.com/vinniefalco/DSPFilters/.

Only RBJ filters are ported over. No introspection is supported & the implementation is quite a different (as Rust would
prefer composition to multiple inheritance).

Very untested, be careful with your speakers.

Depends on `audio-processor-traits`. Exports `FilterProcessor` which may be used for general filtering needs.

See `synth` for example usage.