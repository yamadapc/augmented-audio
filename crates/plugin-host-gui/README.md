# Plugin Host GUI
This is a GUI tool for testing VST plugins based on Tauri, Rust and React.js.

## Installing dependencies
See instructions on the root of this repository for lerna set-up.

## Building for development
```shell
tauri dev
```

## Running the rust app against localhost without build server
```shell
cargo run --no-default-features --package plugin-host-gui
```

## Manually starting the build server
```shell
npm run start
```

## Building for production
```shell
npm run build && cargo build --release
```