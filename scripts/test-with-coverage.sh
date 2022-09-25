#!/usr/bin/env bash

set -e
set -x

cargo llvm-cov nextest --html --ignore-filename-regex="crates/vendor|midir|gui|crates/spikes|daw|development|apps/synth|apps/demo-plugin|crates/scripts" -- --exclude midir --workspace