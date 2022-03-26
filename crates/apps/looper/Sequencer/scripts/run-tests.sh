#!/usr/bin/env bash

rm -rf test-output

./scripts/run-unit-tests.sh

echo "Running Sequencer Mac tests"
xcodebuild -project Sequencer.xcodeproj -scheme "Sequencer Mac" test -resultBundlePath ./test-output/SequencerMac | xcbeautify

# echo "Running Sequencer tests"
#xcodebuild -project Sequencer.xcodeproj -scheme "Sequencer" test -resultBundlePath ./test-output/Sequencer | xcbeautify

xcparse screenshots --os --model --test-plan-config ./test-output/SequencerMac.xcresult ./test-output/SequencerMacScreenshots

screenshot_path=$(find ./test-output/SequencerMacScreenshots | grep "Launch Screen")
mv "$screenshot_path" ./screenshot.png
