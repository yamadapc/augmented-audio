#!/usr/bin/env bash

rm -rf test-output
make test
xcparse screenshots --os --model --test-plan-config ./test-output/SequencerMac.xcresult ./test-output/SequencerMacScreenshots

screenshot_path=$(find ./test-output/SequencerMacScreenshots | grep "Launch Screen")
mv "$screenshot_path" ./screenshot.png