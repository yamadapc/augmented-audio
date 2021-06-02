<p align="center"><img height="150" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/AppIcon%401x.png" /></p>

<h1 align="center">Augmented Audio Libraries</h1>

---

In this repository I'll push some experiments trying to use Rust for audio programming.

## crates/test-plugin-host
Basic test host. Opens a VST plugin and input file, then connects the plugin's
output to the default output device.

## crates/tremolo-plugin
<p align="center"><img height="350" src="https://github.com/yamadapc/rust-audio-software/raw/master/design/tremolo-screenshot.png" /></p>
Incomplete tremolo VST with a WebView GUI.

## crates/oscillator
Basic oscillator implementation.

## crates/example-midi-host
Example MIDI host which will log MIDI messages.

- - -

## JavaScript bits
WebViews are used for UI. A TypeScript browser runtime and front-ends will be included.

Tools will include logging, RPC and so on.
