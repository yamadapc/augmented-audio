#!/usr/bin/env bash

set -x
set -e

example=${1:freeverb}
cargo run --release \
  --example $example -- \
  --input-file ./input-files/bass.wav \
  --output-file /tmp/test.wav

echo "Playing input file for 3s"
afplay -t 3 ./input-files/bass.wav

echo "Playing output file for 3s"
afplay /tmp/test.wav -t 3