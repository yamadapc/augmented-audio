#!/usr/bin/env bash

set -e
set -x

rm -rf test-output

ls ../looper-processor/public
./scripts/run-unit-tests.sh

rm -rf ./test-output/*
echo "Running Sequencer Mac tests"
xcodebuild \
  -project Sequencer.xcodeproj \
  -scheme "Sequencer Mac" \
  -derivedDataPath /tmp/sequencer-derived-data \
  test \
  -resultBundlePath ./test-output/SequencerMac \
  CODE_SIGN_IDENTITY="" CODE_SIGNING_REQUIRED="NO" CODE_SIGNING_ALLOWED="NO" \
  | xcbeautify

echo "Running Sequencer tests"
xcodebuild \
  -project Sequencer.xcodeproj \
  -scheme "Sequencer" \
  test \
  -resultBundlePath ./test-output/Sequencer \
  | xcbeautify
xcodebuild \
  -project Sequencer.xcodeproj \
  -scheme "SequencerUI" \
  test \
  -resultBundlePath ./test-output/SequencerUI \
  | xcbeautify
xcodebuild \
  -project Sequencer.xcodeproj \
  -scheme "SequencerEngine" \
  test \
  -resultBundlePath ./test-output/SequencerEngine \
  | xcbeautify

if [ -x "$(command -v xcparse)" ]; then
  xcparse screenshots --os --model --test-plan-config ./test-output/SequencerMac.xcresult ./test-output/SequencerMacScreenshots
  screenshot_path=$(find ./test-output/SequencerMacScreenshots | grep "Launch Screen")
  mv "$screenshot_path" ./screenshot.png
fi
