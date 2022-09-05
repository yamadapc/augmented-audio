#!/usr/bin/env bash

rm -rf test-output

echo "Running SequencerUI tests"
xcodebuild -project Sequencer.xcodeproj -scheme "SequencerUI" test \
  -derivedDataPath /tmp/sequencer-derived-data \
  -resultBundlePath ./test-output/SequencerUI \
  CODE_SIGN_IDENTITY="" CODE_SIGNING_REQUIRED="NO" CODE_SIGNING_ALLOWED="NO" \
  | xcbeautify

echo "Running SequencerEngine tests"
xcodebuild -project Sequencer.xcodeproj -scheme "SequencerEngine" test \
  -derivedDataPath /tmp/sequencer-derived-data \
  -resultBundlePath ./test-output/SequencerEngine \
  CODE_SIGN_IDENTITY="" CODE_SIGNING_REQUIRED="NO" CODE_SIGNING_ALLOWED="NO" \
  | xcbeautify
