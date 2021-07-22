#!/usr/bin/env bash

set -e
set -x

rm -f default.profraw
rm -f ./*.profraw
rm -f ./crates/**/*.profraw

rm -rf ./coverage/*

RUSTFLAGS="-Zinstrument-coverage" cargo +nightly build --target-dir=./target-nightly

LLVM_PROFILE_FILE="augmented_audio-%p-%m.profraw" RUSTFLAGS="-Zinstrument-coverage" cargo +nightly test --target-dir=./target-nightly

grcov . \
  --binary-path ./target-nightly/debug \
  --source-dir . \
  -t html --branch \
  --ignore ./packages/* \
  --ignore ./crates/baseview \
  --ignore ./crates/cpal \
  --ignore ./crates/iced \
  --ignore ./crates/iced_audio \
  --ignore ./crates/plotters \
  -o ./coverage/

rm -f ./*.profraw
rm -f ./crates/**/*.profraw
