#!/usr/bin/env bash

set -x
set -e

for crate in ./crates/*; do
  cd $crate
  cargo build
  cargo test
  cargo fmt
  cargo clippy
  cd -
done