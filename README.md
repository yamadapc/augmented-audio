<p align="center"><img height="150" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/AppIcon%401x.png" /></p>

<h1 align="center">Augmented Audio Libraries</h1>

[![Default](https://github.com/yamadapc/augmented-audio/actions/workflows/default.yml/badge.svg)](https://github.com/yamadapc/augmented-audio/actions/workflows/default.yml)
[![Linux](https://github.com/yamadapc/augmented-audio/actions/workflows/linux.yml/badge.svg)](https://github.com/yamadapc/augmented-audio/actions/workflows/linux.yml)
[![Web-based builds](https://github.com/yamadapc/augmented-audio/actions/workflows/web.yml/badge.svg)](https://github.com/yamadapc/augmented-audio/actions/workflows/web.yml)
---

In this repository I'll push some experiments trying to use Rust for audio programming.

- - -

## Goals

* **Goal 1:** Learn & have fun
  * This is goal #1 and it's very important to keep it in mind if you end-up
    depending on one of the crates in this repository
* **Goal 2:** Build tools for aiding development
* **Goal 3:** Experiment with Audio software GUI in Rust

## Binary downloads
* [See releases to download binaries](https://github.com/yamadapc/augmented-audio/releases)

- - -

<!--ts-->
* [Augmented Audio Libraries](#augmented-audio-libraries)
   * [Goals](#goals)
* [audio-processor-traits](#audio-processor-traits)
   * [audio-processor-utility](#audio-processor-utility)
   * [atomic-queue](#atomic-queue)
   * [Standalone processor](#standalone-processor)
   * [Standalone MIDI handling](#standalone-midi-handling)
   * [dsp-filters](#dsp-filters)
   * [oscillator](#oscillator)
   * [audio-garbage-collector &amp; audio-garbage-collector-v2](#audio-garbage-collector--audio-garbage-collector-v2)
   * [audio-parameter-store](#audio-parameter-store)
   * [ADSR](#adsr)
   * [Tremolo &amp; Tremolo VST](#tremolo--tremolo-vst)
   * [Looper &amp; Looper VST](#looper--looper-vst)
   * [Synth](#synth)
* [plugin-host - A CLI for hosting VSTs during development](#plugin-host---a-cli-for-hosting-vsts-during-development)
   * [Usage](#usage)
   * [Plugin Host GUI](#plugin-host-gui)
      * [Iced GUI](#iced-gui)
      * [Future things to improve](#future-things-to-improve)
      * [UI elements](#ui-elements)
         * [pick_list](#pick_list)
         * [menu_list](#menu_list)
         * [button](#button)
         * [knobs](#knobs)
         * [sliders](#sliders)
         * [transport](#transport)
* [Web GUI](#web-gui)
* [Rust libraries and tooling](#rust-libraries-and-tooling)
   * [Overall usage of external libraries](#overall-usage-of-external-libraries)
   * [Workspace &amp; Building](#workspace--building)
      * [Building on linux](#building-on-linux)
   * [Linting](#linting)
   * [Benchmarking](#benchmarking)
      * [Profiling on macOS](#profiling-on-macos)
      * [Generating flamegraphs from benchmarks](#generating-flamegraphs-from-benchmarks)
* [Monorepo &amp; Submodules](#monorepo--submodules)

<!-- Added by: yamadapc, at: Thu Jul 22 07:06:11 AEST 2021 -->

<!--te-->

# audio-processor-traits

An abstraction for `AudioProcessor` and `AudioBuffer` implementations.

See [audio-processor-traits](https://github.com/yamadapc/augmented-audio/tree/master/crates/audio-processor-traits) and
its related (work-in-progress) [audio-processor-graph](https://github.com/yamadapc/augmented-audio/tree/master/crates/audio-processor-graph).

```rust
pub trait AudioProcessor {
    type SampleType;
    fn prepare(&mut self, _settings: AudioProcessorSettings) {}
    fn process<BufferType: AudioBuffer<SampleType = Self::SampleType>>(
        &mut self,
        data: &mut BufferType,
    );
}
```

## audio-processor-utility
[Panning, gain, mono/stereo processors.](https://github.com/yamadapc/augmented-audio/tree/master/crates/audio-processor-utility)

## atomic-queue
[A multi-producer/multi-consumer bounded lock-free queue.](https://github.com/yamadapc/augmented-audio/tree/master/crates/atomic-queue)

## Standalone processor
Implementing the trait enables easy stand-alone hosting of an audio app: [`audio-processor-standalone`](https://github.com/yamadapc/augmented-audio/tree/master/crates/audio-processor-standalone).

## Standalone MIDI handling
Implementing the trait enables easy stand-alone MIDI handling: [`audio-processor-standalone-midi`](https://github.com/yamadapc/augmented-audio/tree/master/crates/audio-processor-standalone-midi).

## `dsp-filters`
[A port of the RJB filters in Vinnie Falco's C++ DSPFilters library. Contains resonant low-pass, high-pass, band-pass,
shelf etc. & implements the `AudioProcessor` trait.](https://github.com/yamadapc/augmented-audio/tree/master/crates/dsp-filters)

## oscillator
[Basic oscillator implementation.](https://github.com/yamadapc/augmented-audio/tree/master/crates/oscillator)

## audio-garbage-collector & audio-garbage-collector-v2
These are wrappers on `basedrop` & my own WIP implementation of smart pointers that do reference counting but are
deallocated on a background thread so they're safe to use the audio-thread.

## audio-parameter-store
Implementation of a "parameter store" for audio plugins. Holds audio plugin parameters in a rw locked hashmap and uses
atomics on parameter values.

This needs to be reviewed as the locks could be avoided all together & it might not be real-time safe to acquire the
lock.

## ADSR
Basic ADSR envelope implementation.

## Tremolo & Tremolo VST
Basic tremolo with web GUI

## Looper & Looper VST
WIP looper implementation.

## Synth
Basic synth implementation to show-case `audio-processor-traits` & other crates.

# plugin-host - A CLI for hosting VSTs during development
[plugin-host-cli](https://github.com/yamadapc/augmented-audio/tree/master/crates/plugin-host-cli) is a CLI tool for
testing VST plug-ins.

It's a simple VST host which can open a plug-in and play an audio file through it in a loop. Additionally, it supports
watching the VST plug-in for changes & reloading it any time it changes.

It also supports offline rendering into a file and printing some basic diagnostics.

## Usage
To run a plug-in looping over a file, on your computer's default audio output, run:
```shell
plugin-host run --plugin ./target/release/myplugin.dylib --input ./my-input-file.mp3
```

To host the plug-in's GUI, use the `--editor` flag:
```shell
plugin-host run --editor --plugin ./target/release/myplugin.dylib --input ./my-input-file.mp3
```

To watch the plug-in dylib for changes, use the `--watch` flag:
_(This currently won't work with an editor)_

```shell
plugin-host run --watch --plugin ./target/release/myplugin.dylib --input ./my-input-file.mp3
```

To run off-line rendering to a file, use the `--output` flag:
```shell
plugin-host run --output ./output.wav --plugin ./target/release/myplugin.dylib --input ./my-input-file.mp3
```

## Plugin Host GUI
### Iced GUI
<p align="center"><img height="350" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/iced-screenshot.png" /></p>

[plugin-host-gui2](https://github.com/yamadapc/augmented-audio/tree/master/crates/plugin-host-gui2) is a GUI for the
testing host.

Features supported in the GUI:

* Loading the VST - Including the GUI, but with caveats
* Selecting audio IO options (input/output/driver)
* Selecting an input file
* Transport controls
* File watcher. The plugin and its editor will be reloaded on re-build.

Missing functionality:

* Volume metering/control (see #16)

### Future things to improve

* Implement limiter
* Show some basic output visualizations for analysis
* Allow using the default input rather than just input files
* Implement offline rendering

### UI elements
Styles on top of `iced_audio` & `iced`, see [`audio-processor-iced-design-system`](https://github.com/yamadapc/augmented-audio/tree/master/crates/audio-processor-iced-design-system).

#### `volume_meter`
<p align="center"><img height="250" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/ui/volume.png" /></p>

Volume control with visualisation.

#### `pick_list`
<p align="center"><img height="250" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/ui/picklist.png" /></p>

Iced `pick_list` / dropdown menu with a label.

#### `menu_list`
<p align="center"><img height="250" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/ui/menu_list.png" /></p>

#### `button`
<p align="center"><img height="250" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/ui/button.png" /></p>

#### `knobs`
<p align="center"><img height="250" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/ui/knobs.png" /></p>

#### `sliders`
<p align="center"><img height="250" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/ui/sliders.png" /></p>

#### `transport`
<p align="center"><img height="100" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/ui/transport.png" /></p>

# Web GUI
See `docs/WEB_GUI.md`.

# Rust libraries and tooling

## Overall usage of external libraries
This uses `rust-vst` to build VSTs. For GUI, there're two strategies in place for web based GUI:

* `tauri` is used for `plugin-host-gui` - Stand-alone
* Raw bindings into `webkit` are used for VST plug-ins - This may change as I consider pros/cons of this approach

Due to `tauri` bundling front-end assets into the `plugin-host-gui` binary, its front-end app needs to have been built
prior to executing its build command in release mode.

## Workspace & Building
The project is set-up with a cargo workspace. Running `cargo` commands at the root directory should compile all crates
sharing caches.

To build the whole project run:
```shell
cargo build
```

To run tests:
```shell
cargo test
```

Build outputs should be on `target/debug` or `target/release`.

### Building on linux
Since this is bringing in all the possible rust crates, you'll need to install quite a few dependencies.

See `.github/workflows/default.yml` for a list of what's needed on Ubuntu.

## Linting
```shell
cargo clippy
```

## Benchmarking
Benchmarks using `criterion` will be slowly added. In order to run benchmarks use:
```shell
cargo bench
```

### Profiling on macOS
> https://crates.io/crates/cargo-instruments

```shell
cd ./crates/oscillator
cargo instruments -t time --bench sine_oscillator_benchmark -- --bench
```

### Generating flamegraphs from benchmarks
> **NOTE** I couldn't get this to work on macOS

Flamegraphs can be generated using `cargo-flamegraph`:
```shell
cargo install flamegraph
```

The tool can then be used to run a criterion benchmark and generate a flamegraph:
```shell
cargo flamegraph --bench sine_oscillator_benchmark -- --bench
```

# Monorepo & Submodules
This is a mono-repository and several dependencies are vendored as forked submodules. A script can sync the upstreams:
```shell
cargo run --package autosync-submodules
```

This will fetch all upstreams, list the new commits & try to merge them.
