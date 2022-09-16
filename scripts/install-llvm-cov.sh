#!/usr/bin/env bash

set -e

host=$(rustc -Vv | grep host | sed "s/host: //")

echo $host

curl -LsSf "https://github.com/taiki-e/cargo-llvm-cov/releases/latest/download/cargo-llvm-cov-$host.tar.gz" | \
  tar xzf - -C $HOME/.cargo/bin
