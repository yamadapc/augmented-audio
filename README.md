<p align="center"><img height="150" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/AppIcon%401x.png" /></p>

<h1 align="center">Augmented Audio Libraries</h1>

[![Default](https://github.com/yamadapc/augmented-audio/actions/workflows/default.yml/badge.svg)](https://github.com/yamadapc/augmented-audio/actions/workflows/default.yml)
[![Linux](https://github.com/yamadapc/augmented-audio/actions/workflows/linux.yml/badge.svg)](https://github.com/yamadapc/augmented-audio/actions/workflows/linux.yml)
[![Web-based builds](https://github.com/yamadapc/augmented-audio/actions/workflows/web.yml/badge.svg)](https://github.com/yamadapc/augmented-audio/actions/workflows/web.yml)
[![Coverage Status](https://coveralls.io/repos/github/yamadapc/augmented-audio/badge.svg?branch=master)](https://coveralls.io/github/yamadapc/augmented-audio?branch=master)
---

Experiments trying to use Rust for audio programming.

## Goals
* **Goal 1:** Learn & have fun
  * This is goal #1 and it's very important to keep it in mind if you end-up
    depending on one of the crates in this repository
* **Goal 2:** Build tools for aiding development
* **Goal 3:** Experiment with Audio software GUI in Rust

## Binary downloads
* [See releases to download binaries](https://github.com/yamadapc/augmented-audio/releases)
* [Simple Metronome on App Store](https://apps.apple.com/au/app/simple-metronome/id1604183938?mt=12)

## Blog posts
* [Initial 'Test Plugin Host' post](https://beijaflor.io/blog/07-2021/rust-audio-experiments-2/)
* [Simple Metronome release](https://beijaflor.io/blog/01-2022/rust-audio-experiments-3/)

## Documentation

* [Augmented Audio Libraries](crates/augmented#readme)
* [Applications in this repository](crates/apps#readme)
  - [Test plugin host](crates/apps/plugin-host#readme)
  - [Metronome](crates/apps/metronome#readme)
  - [Looper](crates/apps/looper#readme)

## Screenshots

<p align="center" style="display: flex;">
  <img alt="Metronome screenshot" src="https://github.com/yamadapc/augmented-audio/raw/master/crates/apps/metronome/design/screenshots/single.png" height="200" />
  <img alt="Test plugin host screenshot" src="https://github.com/yamadapc/augmented-audio/raw/master/crates/apps/plugin-host/screenshot.png" width="300" />
  <img alt="Looper screenshot" src="https://github.com/yamadapc/augmented-audio/raw/master/crates/apps/looper/screenshot.png" width="300" />
</p>

<p align="center" style="display: flex;">
  <img height="100" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/ui/volume.png" />
  <img width="100" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/ui/picklist.png" />
  <img height="100" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/ui/menu_list.png" />
  <img height="100" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/ui/button.png" />
  <img height="100" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/ui/knobs.png" />
  <img height="100" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/ui/sliders.png" />
  <img width="100" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/ui/transport.png" />
  <img height="100" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/tremolo-screenshot.png" />
</p>

# Web GUI
See [`docs/misc/WEB_GUI.md`](docs/misc/WEB_GUI.md).

# Rust libraries and tooling
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

### Snapshot testing audio processors
See [`docs/monorepo-tooling/SNAPSHOT_TESTING.md`](docs/monorepo-tooling/SNAPSHOT_TESTING.md).
