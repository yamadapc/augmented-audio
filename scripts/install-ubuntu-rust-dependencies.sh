#!/usr/bin/env bash
set -e

if ! [ -x "$(command -v uniffi-bindgen)" ]; then
  cargo install --force --version 0.15.2 uniffi_bindgen
fi

if ! [ -x "$(command -v cargo-tarpaulin)" ]; then
  cargo install --force cargo-tarpaulin
fi

if ! [ -x "$(command -v cargo-nextest)" ]; then
  cargo install --force cargo-nextest
fi
