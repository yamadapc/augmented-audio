#!/usr/bin/env bash

set -e

CARGO_PKG_VERSION=$(cat ./crates/plugin-host-gui2/Cargo.toml | grep version | head -n1 | awk '{ print $3 }' | sed 's/"//g')
GIT_REV=$(git rev-parse --short HEAD)
version="release-$CARGO_PKG_VERSION-$GIT_REV"

echo ">> Building plugin-host-gui2 version=$version"
cargo build --release --package plugin-host-gui2

echo ">> Building \"Plugin Host.app\" bundle"
mkdir -p ./target/apps/macos
cp -r "./crates/plugin-host-gui2/macos-app/Plugin Host.app" ./target/apps/macos/
cp target/release/plugin-host-gui2 "./target/apps/macos/Plugin Host.app/Contents/MacOS/"

