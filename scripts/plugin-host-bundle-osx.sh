#!/usr/bin/env bash

set -e
set -x

cargo build --release --package plugin-host-gui2
mkdir -p ./target/apps/macos
cp -r "./crates/plugin-host-gui2/macos-app/Plugin Host.app" ./target/apps/macos/
cp target/release/plugin-host-gui2 "./target/apps/macos/Plugin Host.app/Contents/MacOS/"