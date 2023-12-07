#!/usr/bin/env bash
set -e

if ! [ -x "$(command -v cargo-tarpaulin)" ]; then
  cargo install cargo-tarpaulin
fi

if ! [ -x "$(command -v cargo-llvm-cov)" ]; then
  # Get host target
  host=$(rustc -Vv | grep host | sed 's/host: //')
  # Download binary and install to $HOME/.cargo/bin
  curl -LsSf https://github.com/taiki-e/cargo-llvm-cov/releases/latest/download/cargo-llvm-cov-$host.tar.gz | tar xzf - -C $HOME/.cargo/bin
fi

#if ! [ -x "$(command -v cargo-nextest)" ]; then
#  curl -LsSf https://get.nexte.st/latest/linux | tar zxf - -C ${CARGO_HOME:-~/.cargo}/bin
#fi
