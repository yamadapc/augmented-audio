#!/usr/bin/env bash

rm -rf test-output

echo "Running SequencerUI tests"
xcodebuild -project Sequencer.xcodeproj -scheme "SequencerUI" test -resultBundlePath ./test-output/SequencerUI | xcbeautify

echo "Running SequencerEngine tests"
xcodebuild -project Sequencer.xcodeproj -scheme "SequencerEngine" test -resultBundlePath ./test-output/SequencerEngine | xcbeautify
