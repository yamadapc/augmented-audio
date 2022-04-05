#!/usr/bin/env bash
set -e

cargo clippy --features story --workspace \
  --no-deps \
  --exclude skribo \
  --exclude augmented-ui \
  --exclude midir \
  --exclude assert_no_alloc \
  --exclude avfaudio-sys
