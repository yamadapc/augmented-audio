#!/usr/bin/env bash

set -e
set -x

rm -f default.profraw

RUSTFLAGS="-Zinstrument-coverage" cargo +nightly test --target-dir=./target-nightly

grcov . \
  --binary-path ./target-nightly/debug \
  --source-dir . \
  -t html --branch \
  -o "./coverage/"
