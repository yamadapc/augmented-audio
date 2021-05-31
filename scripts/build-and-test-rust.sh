#!/usr/bin/env bash

set -x
set -e

cargo build
cargo test
cargo fmt
cargo clippy
