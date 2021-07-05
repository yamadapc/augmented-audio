<p align="center"><img height="150" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/AppIcon%401x.png" /></p>

<h1 align="center">Augmented Audio Libraries</h1>

[![Default](https://github.com/yamadapc/augmented-audio/actions/workflows/default.yml/badge.svg)](https://github.com/yamadapc/augmented-audio/actions/workflows/default.yml)
---

In this repository I'll push some experiments trying to use Rust for audio programming.

- - -

## Goals of this repository

* **Goal 1:** Learn & have fun
* **Goal 2:** Build tools for aiding development
* **Goal 3:** Experiment with Audio software GUI in Rust

## audio-processor-traits
See [audio-processor-traits](https://github.com/yamadapc/augmented-audio/tree/master/crates/audio-processor-traits).

## plugin-host - A CLI for hosting VSTs
`plugin-host` is a CLI tool for testing VST plug-ins. It's a simple VST host which can open a plug-in and play an audio
file through it in a loop. Additionally, it supports watching the VST plug-in for changes & reloading it any time it
changes.

It also supports offline rendering into a file, with some basic diagnostics being printed.

### Usage
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

### GUI
There's also a GUI for this (see more later on)
<p align="center"><img height="350" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/host-screenshot.png" /></p>

Features supported in the GUI:

* Loading the VST
* Selecting audio IO options (input/output/driver)
* Selecting an input file
* Transport controls
* Volume metering

Missing functionality:

* File watcher (works in CLI, but not in host GUI see #15)
* Volume control (needs to be wired-up see #16)

### Future things to improve

* Implement limiter
* Show some basic output visualizations for analysis

## Architecture of web based VST GUI
<p align="center">
  <img height="400" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/web-gui-diagram.png" />
</p>

**Note:** There are many reasons why web-based GUI is bad in this use-case, for example:

- High-complexity of infrastructure to support runtime & deployment
- Unavoidably worse performance at certain tasks
- Overall not worth it, assuming there's are nice cross-platform Rust GUI frameworks and/or the GUI is not too complex

However, there are also reasons why it could be good:

- Amazing development experience: hot-reload, React, Chrome dev-tools
- Potential for having plug-in GUIs that run in a browser
  * This can be: remote control for Bela, iPads; a Web front-end for a WebAssembly version of the plug-in, etc.
  * Without any changes to native/javascript code (because we ate the complexity by doing this)
    
So I'm just trying it out & seeing where it can go.

### Main components
The web based Tremolo VST in `crates/tremolo-plugin` contains two main components:

* React.js JavaScript front-end
* Rust VST plug-in

These two sides communicate via messages.

### Audio thread
On the audio side, the VST is hosted by a certain DAW application, which will call its process callbacks on an audio
thread. It'll read its state from a "parameter store". This is a very simple general purpose shared hashmap that uses
atomics internally. This could be any lock-free or real-time safe shared data-structure.

### GUI thread
If the Host creates an editor window via the VST API, the plug-in will pass an editor instance back. These are views
and their callbacks should run on the GUI thread managed by the host. Within its window, the plug-in will host a webkit
view with the JavaScript app and interact with it through an abstract transport layer.

#### Webview transport layer
The "WebViewTransportLayer" is a channel based layer which will allow pushing messages in/out of the webview through
channels from any thread. There're two different transports:

* Webkit message handlers transport ; evaluating JavaScript to send messages, which should be used on a release build
* WebSockets transport, which enables remote control of the plug-in from a browser (for development) or through
  the network (if we wanted)

#### Tokio
Due to this transport component, Tokio is used to run multiple queue processing loops on a few threads, as well as
providing the websockets async server functionality.

### JavaScript & message passing overhead
The JavaScript side has the same abstract transport layer to support the client-side counterparts of both webkit message
handlers & websockets behind the same API.

Due to messages being JSON, they need to be serialized/deserialized at both ends. This has some amount of overhead if
messages are frequent.

## Notes on `tauri` based `plugin-host-gui`
I'm playing with using `tauri` for a web based `plugin-host` GUI. This has some interesting issues such as making
`plugin-host` able to change some of its state (such as its audio output) on the fly.

One simple thing which will not work optimally with web GUI is trying to do audio visualization. What I tried to do was
passing a very simple `volume` tuple and visualize it.

The use case here is a very simple bit of audio GUI: A volume meter. This is now working in `plugin-host-gui` with a
series of little caveats.

In order to prevent high CPU usage I've measured the following compromises and approaches to be the best:

1. Don't do 60fps rendering through JavaScript - Even `requestAnimationFrame` doing no work has relatively high overhead
2. Instead, push animation to browser via `transition` & animate via CSS `transform` changes
3. `tauri` built-in event system has high CPU usage ; it's much more efficient to call `evaluateJavascript` directly &
   set the volume on global variables
4. Both JavaScript & Rust will poll/set the volume via polling

Regarding JavaScript/CSS rendering. Approaches that I tried:

* Canvas
* WebGL
* CSS & transforms - With CSS style/layout/paint containment enabled

Ultimately the last one won, which is a shame, because it's the least flexible & relies on the GUI interpolating values.

This is more efficient because JavaScript callbacks don't run at 30fps/60fps, but at a rather slow pace (100ms) where
they'll update styles ; a transition on those styles renders the animation.

Currently, this means the GUI is always behind the actual value by a significant amount. Because it takes at least 200ms
for the meter to reach a volume target value after it was read.

Some tweaking of the transition times & JavaScript poll logic could improve this, but generally it's not a great
situation.

## Bundling
Under `crates/bundler` there's a basic program which will take care of bundling the app as a `.vst` bundle for macOS.
This is just a valid `Info.plist` as well as copying front-end resources into the package.

The VST will access resource paths from its bundle using `CFBundle` functions. A missing binding is added in
`macos-bundle-resources` to get resources in the plug-in bundle.

See:
* https://github.com/yamadapc/augmented-audio/tree/master/crates/bundler
* https://github.com/yamadapc/augmented-audio/tree/master/crates/macos-bundle-resources

## Standalone MIDI handling
See `audio-processor-standalone-midi`.

## crates/plugin-host-cli
```
plugin-host run \
    # Open VST at plugin.dylib
    --plugin ./plugin.dylib \
    # Open, decode & convert sample-rate of input.mp3
    --input ./input.mp3 \
    # Open window & host the VST GUI
    --editor
```
Basic test host. Opens a VST plugin and input file, then connects the plugin's
output to the default output device.

## crates/tremolo-plugin
<p align="center"><img height="350" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/tremolo-screenshot.png" /></p>

Communicates via messages with the Rust audio processor.

### Building the tremolo-plugin
```shell
cargo run --package ruas-bundler -- \
    --config ./crates/tremolo-plugin/Cargo.toml \
    --output ./target/vsts/tas.vst \
    --frontend-path ./packages/tremolo-plugin-frontend
```

This will build the VST & its front-end and generate a working `target/vsts/tas.vst` bundle.

## crates/audio-parameter-store
Implementation of a "parameter store" for audio plugins. Holds audio plugin parameters in a rw locked hashmap and uses
atomics on parameter values.

## crates/webview-transport
Abstraction for messaging with JavaScript webview. Provides a websockets & webkit message handler based transports.

On development, websockets may be used. This allows for the UI to be developed on Google Chrome rather than the
embedded webview. In production webkit message handlers may be used.

Front-end has a corresponding package in `packages/webview-transport`.

## crates/webview-holder
A wrapper on top of webkit webview for MacOS.

## crates/oscillator
Basic oscillator implementation.

## crates/example-midi-host
Example MIDI host which will log MIDI messages.

## Rust libraries and tooling

### Overall usage of external libraries
This uses `rust-vst` to build VSTs. For GUI, there're two strategies in place for web based GUI:

* `tauri` is used for `plugin-host-gui` - Stand-alone
* Raw bindings into `webkit` are used for VST plug-ins - This may change as I consider pros/cons of this approach

Due to `tauri` bundling front-end assets into the `plugin-host-gui` binary, its front-end app needs to have been built
prior to executing its build command in release mode.

### Workspace & Building
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

### Linting
```shell
cargo clippy
```

### Benchmarking
Benchmarks using `criterion` will be slowly added. In order to run benchmarks use:
```shell
cargo bench
```

#### Profiling on macOS
> https://crates.io/crates/cargo-instruments

```shell
cd ./crates/oscillator
cargo instruments -t time --bench sine_oscillator_benchmark -- --bench
```

#### Generating flamegraphs from benchmarks
> **NOTE** I couldn't get this to work on macOS

Flamegraphs can be generated using `cargo-flamegraph`:
```shell
cargo install flamegraph
```

The tool can then be used to run a criterion benchmark and generate a flamegraph:
```shell
cargo flamegraph --bench sine_oscillator_benchmark -- --bench
```

## JavaScript bits
WebViews are used for UI. A TypeScript browser runtime and front-ends will be included.

Tools will include logging, RPC and so on.
