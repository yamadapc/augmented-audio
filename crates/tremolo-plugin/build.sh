#!/usr/bin/env bash

set -x
set -e

cargo build
./osx_bundle.sh TasV2 ./target/debug/libtas_v2.dylib
rm -rf ~/Library/Audio/Plug-Ins/VST/TasV2.vst/
cp -r TasV2.vst ~/Library/Audio/Plug-Ins/VST/
mv ./TasV2.vst ./target/
