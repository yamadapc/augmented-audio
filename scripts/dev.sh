#!/usr/bin/env bash
set -e
cargo run --release --package augmented-dev-cli -- $@
