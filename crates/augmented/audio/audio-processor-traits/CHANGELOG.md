## v4.0.0

* [`1d6c549959a6`](https://github.com/yamadapc/augmented-audio/commits/1d6c549959a6) audio-garbage-collector@1.2.0 (Minor)
* [`a4d5cd74f0b6`](https://github.com/yamadapc/augmented-audio/commits/a4d5cd74f0b6) Add missing preambles (Minor)
* [`82205083bd1b`](https://github.com/yamadapc/augmented-audio/commits/82205083bd1b) Fix clippy errors (Minor)
* [`31a79c30fab4`](https://github.com/yamadapc/augmented-audio/commits/31a79c30fab4) Fix unit-tests and issues (Minor)
* [`b31bd0c432e9`](https://github.com/yamadapc/augmented-audio/commits/b31bd0c432e9) Bump other crates like plugin host :major: (Major)
* [`37e0d2e25d83`](https://github.com/yamadapc/augmented-audio/commits/37e0d2e25d83) Update synth :major: (Major)
* [`9971b92d97bc`](https://github.com/yamadapc/augmented-audio/commits/9971b92d97bc) Update looper processor :major: (Major)
* [`f9e735c3424c`](https://github.com/yamadapc/augmented-audio/commits/f9e735c3424c) Update dynamics package :major: (Major)
* [`dac22f5589d5`](https://github.com/yamadapc/augmented-audio/commits/dac22f5589d5) Fix metronome with new traits :major: (Major)
* [`3fb9588a86d4`](https://github.com/yamadapc/augmented-audio/commits/3fb9588a86d4) Other fixes for utility, traits and file :major: (Major)
* [`babead77eaff`](https://github.com/yamadapc/augmented-audio/commits/babead77eaff) Bump processor file :major: (Major)
* [`f941e8f46d01`](https://github.com/yamadapc/augmented-audio/commits/f941e8f46d01) Bump analysis crate :major: (Major)
* [`46e71da4fe24`](https://github.com/yamadapc/augmented-audio/commits/46e71da4fe24) Move on with refactoring (Minor)
* [`8bb0d3cdde41`](https://github.com/yamadapc/augmented-audio/commits/8bb0d3cdde41) Get filters crate to compile and test (Minor)
* [`62519e8786ad`](https://github.com/yamadapc/augmented-audio/commits/62519e8786ad) Update with buffer struct (Minor)
* [`75ed94ed1e4f`](https://github.com/yamadapc/augmented-audio/commits/75ed94ed1e4f) Update with vec buffers (Minor)
* [`081029b82405`](https://github.com/yamadapc/augmented-audio/commits/081029b82405) Initial trait changes (Minor)
* [`2a2e40d2d0be`](https://github.com/yamadapc/augmented-audio/commits/2a2e40d2d0be) Update with more unit-tests (Minor)
* [`1900a56e7aac`](https://github.com/yamadapc/augmented-audio/commits/1900a56e7aac) Update with more unit-tests for traits (Minor)
* [`3eb85ff9540a`](https://github.com/yamadapc/augmented-audio/commits/3eb85ff9540a) Add more unit-tests (Minor)
* [`43c9edde3992`](https://github.com/yamadapc/augmented-audio/commits/43c9edde3992) Update with noop processor tests (Minor)
* [`dff46ef6c5c5`](https://github.com/yamadapc/augmented-audio/commits/dff46ef6c5c5) Keep adding unit-tests (Minor)
* [`82ca74d2ce9f`](https://github.com/yamadapc/augmented-audio/commits/82ca74d2ce9f) Add audioBuffer trait tests (Minor)
* [`ef9be8a9214c`](https://github.com/yamadapc/augmented-audio/commits/ef9be8a9214c) Fix default features for compilation (Minor)
* [`b88cfb99aaa8`](https://github.com/yamadapc/augmented-audio/commits/b88cfb99aaa8) Change default sample rate conversion (Minor)

## v3.2.0

* [`ce67876edbf`](https://github.com/yamadapc/augmented-audio/commits/ce67876edbf) augmented_oscillator@1.2.1 (Minor)
* [`93d44428a9a`](https://github.com/yamadapc/augmented-audio/commits/93d44428a9a) audio-garbage-collector@1.1.1 (Minor)
* [`90164dc4223`](https://github.com/yamadapc/augmented-audio/commits/90164dc4223) Update with a new benchmark (Minor)
* [`1ef3e6a1fe1`](https://github.com/yamadapc/augmented-audio/commits/1ef3e6a1fe1) Add basic combinators (Minor)
* [`9a78e0bcc61`](https://github.com/yamadapc/augmented-audio/commits/9a78e0bcc61) Start to add context parameter and combinators :major: (Major)
* [`cd475f5e06d`](https://github.com/yamadapc/augmented-audio/commits/cd475f5e06d) Fix clippy warnings :patch: (Patch)

## v2.2.0

* [`9f02ad99ff`](https://github.com/yamadapc/augmented-audio/commits/9f02ad99ff) augmented_oscillator@1.2.0 (Minor)
* [`035499207e`](https://github.com/yamadapc/augmented-audio/commits/035499207e) augmented-atomics@0.1.2 (Minor)
* [`d8684c483c`](https://github.com/yamadapc/augmented-audio/commits/d8684c483c) Add helper to create vec audio buffer :minor: (Minor)

## v2.1.0

* [`665a0e189`](https://github.com/yamadapc/augmented-audio/commits/665a0e189) Remove vst buffer and add buffer handler :breaking: (Major)

## v1.1.0

* [`7e397e13a`](https://github.com/yamadapc/augmented-audio/commits/7e397e13a) audio-garbage-collector@1.1.0 (Minor)
* [`b5023a027`](https://github.com/yamadapc/augmented-audio/commits/b5023a027) augmented_oscillator@1.1.0 (Minor)
* [`2d281455c`](https://github.com/yamadapc/augmented-audio/commits/2d281455c) augmented-atomics@0.1.1 (Minor)
* [`07fa62a64`](https://github.com/yamadapc/augmented-audio/commits/07fa62a64) Bump vst and update documentation actions (Minor)
* [`4fc4eec35`](https://github.com/yamadapc/augmented-audio/commits/4fc4eec35) Bump dependencies & fix tests (Patch)

## 0.3.1
* Made audio-processor-settings fields public

## 0.3.0
* Relaxed sync / send bounds
* Deprecated `VSTAudioBuffer`
* Added `slice` / `slice_mut` family of helpers

## 0.2.0
* Add MIDI processor support
* Improve documentation

## 0.1.0
* Initial release