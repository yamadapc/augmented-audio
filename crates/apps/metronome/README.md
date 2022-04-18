<p align="center">
  <img src="design/icon.png" width="150" />
</p>

# Simple Metronome

This is a tiny metronome app, meant to exercise `augmented-audio` libraries a bit.

You can read about it on: https://beijaflor.io/blog/01-2022/rust-audio-experiments-3/

And you can download it from the app store on [Simple Metronome](https://apps.apple.com/au/app/simple-metronome/id1604183938?mt=12).

<p align="center">
  <img src="https://github.com/yamadapc/augmented-audio/raw/master/crates/apps/metronome/design/screenshots/01-main.png" width="400" />
</p>

<p align="center" style="display: flex;">
  <img src="https://github.com/yamadapc/augmented-audio/raw/master/crates/apps/metronome/design/screenshots/02-features.png" width="300" />
  <img src="https://github.com/yamadapc/augmented-audio/raw/master/crates/apps/metronome/design/screenshots/03-dark.png" width="300" />
</p>

## Generating bridge
```
cargo install flutter_rust_bridge_codegen --version "^1.13"
```

Then:
```
make build-bindings
```

If on a M1 mac, you will need x86 homebrew and LLVM installed, since flutter 
tooling will only run via rosetta. See - https://stackoverflow.com/questions/67386941/using-x86-libraries-and-openmp-on-macos-arm64-architecture.

## License
This subdirectory is licensed under AGPLv3 for now.
