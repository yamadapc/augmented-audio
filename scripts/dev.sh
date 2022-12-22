#!/usr/bin/env bash
set -e
set -x
cargo run --release --package augmented-dev-cli -- $@
