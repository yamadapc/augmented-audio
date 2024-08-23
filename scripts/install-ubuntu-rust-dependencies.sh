#!/usr/bin/env bash
set -e

curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
cargo binstall cargo-tarpaulin
cargo binstall cargo-llvm-cov
cargo binstall cargo-nextest