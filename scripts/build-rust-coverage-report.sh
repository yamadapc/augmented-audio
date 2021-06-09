#!/usr/bin/env bash

set -e
set -x

rm -f default.profraw

RUSTFLAGS="-Zinstrument-coverage" cargo +nightly test

grcov . \
  --binary-path ./target/debug \
  --source-dir . \
  -t html --branch \
  -o "./coverage/"
