#!/usr/bin/env bash

set -e
set -x

rm -f default.profraw

RUSTFLAGS="-Zinstrument-coverage" cargo +nightly build
RUSTFLAGS="-Zinstrument-coverage" cargo +nightly test

grcov . \
  --binary-path ./target/debug \
  --source-dir . \
  -t html --branch \
  --ignore-not-existing \
  -o "./coverage/"
