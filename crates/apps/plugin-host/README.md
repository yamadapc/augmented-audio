# plugin-host - A testing host for VST development
<p align="center">
  <img src="https://github.com/yamadapc/augmented-audio/raw/master/crates/apps/plugin-host/screenshot.png" width="400" />
</p>

[plugin-host-cli](https://github.com/yamadapc/augmented-audio/tree/master/crates/apps/plugin-host/plugin-host-cli) is a CLI tool for
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

[plugin-host-gui2](https://github.com/yamadapc/augmented-audio/tree/master/crates/apps/plugin-host/plugin-host-gui2) is a GUI for the
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
