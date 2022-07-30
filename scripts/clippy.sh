#!/usr/bin/env bash
set -e

cargo clippy --features story --workspace \
  --no-deps \
  --exclude midir \
  --exclude avfaudio-sys
