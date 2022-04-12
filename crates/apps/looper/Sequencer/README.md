<p align="center">
  <img src="design/icon/Icon@128h.png" style="max-width: 128px" />
</p>

<h1 align="center">Continuous Looper</h1>
<p align="center">
    <strong>live-looper and performance sampler</strong>
    <strong> [VIDEO DEMO](https://youtu.be/PcXRXFE9_So)</strong>
</p>

* 8 track looper
* Step sequence recorded loops
* Automatic slicing based on transient detection
* Parameter lock into step parameters
* Scene support
* Multiple quantization modes
* Easy, click and twist MIDI mapping

![](screenshot.png)

Read about it here:

* https://beijaflor.io/blog/04-2022/rust-audio-experiments-5/

Download it here:

* https://apps.apple.com/au/app/continuous-looper/id1616355791?mt=12

## Building

This app requires `looper-processor` universal C library bindings to be built:

```
cd ./crates/apps/looper/looper-processor
make
```

Then the app should be able to build.

## Parsing test results
```
brew install chargepoint/xcparse/xcparse
```

## License
This sub-directory is licensed under the AGPLv3 license.
