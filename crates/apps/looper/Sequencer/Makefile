format: FORCE
	swiftformat ./Sequencer/**/*.swift
	swiftformat ./SequencerUITests/**/*.swift
	swiftformat ./Sequencer\ AudioUnit/**/*.swift
	swiftformat ./SequencerUI/**/*.swift
	swiftformat ./Sequencer\ Mac/**/*.swift
	swiftformat ./Sequencer\ MacUITests/**/*.swift
	swiftformat ./SequencerEngine/**/*.swift
	python3 ./scripts/add_license_preamble.py

build: FORCE
	cd ../looper-processor && make
	xcodebuild -project Sequencer.xcodeproj -scheme "Sequencer Mac" build

test: FORCE
	./scripts/run-tests.sh

FORCE:
