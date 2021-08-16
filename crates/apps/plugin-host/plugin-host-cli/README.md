# plugin-host
`plugin-host` is a CLI tool for testing VST plug-ins.

It's a simple VST host which can open a plug-in and for testing purposes.

## Features

* Loop an audio file through the plugin
* Run the plugin through an audio file & render the contents to a `.wav` result
* Watching the plug-in for changes while looping over the file & reloading if its rebuilt
* Opening a window to host the plug-in
* Forwarding MIDI events to the plug-in

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

