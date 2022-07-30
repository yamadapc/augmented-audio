#!/usr/bin/env bash
set -e

if ! [ -x "$(command -v uniffi-bindgen)" ]; then
  cargo install --version 0.15.2 uniffi_bindgen
fi

if ! [ -x "$(command -v cargo-tarpaulin)" ]; then
  cargo install cargo-tarpaulin
fi
