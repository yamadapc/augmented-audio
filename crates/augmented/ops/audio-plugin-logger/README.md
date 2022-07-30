# audio-plugin-logger

Preset configuration for `log4rs` inside of a VST plugin.

`get_configuration_root_path()` will return the user $HOME/.ruas directory.

`init("logger-name")` will set-up logging within this directory. Logs will rotate if they are
over 10MB. The directories will be created automatically.

License: MIT
