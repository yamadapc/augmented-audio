format: FORCE
	swiftformat ./Sequencer/**/*.swift
	swiftformat ./SequencerUITests/**/*.swift
	swiftformat ./Sequencer\ AudioUnit/**/*.swift
	swiftformat ./SequencerUI/**/*.swift
	swiftformat ./Sequencer\ Mac/**/*.swift
	swiftformat ./Sequencer\ MacUITests/**/*.swift
	swiftformat ./SequencerEngine/**/*.swift
	python3 ./scripts/add_license_preamble.py

test: FORCE
	@echo "Checking if xcbeautify is installed"
	$(`which xcbeautify` || brew install xcbeautify)
	@echo "Running SequencerUI tests"
	xcodebuild -project Sequencer.xcodeproj -scheme "SequencerUI" test -resultBundlePath ./test-output/SequencerUI | xcbeautify
	@echo "Running SequencerEngine tests"
	xcodebuild -project Sequencer.xcodeproj -scheme "SequencerEngine" test -resultBundlePath ./test-output/SequencerEngine | xcbeautify
	@echo "Running Sequencer Mac tests"
	xcodebuild -project Sequencer.xcodeproj -scheme "Sequencer Mac" test -resultBundlePath ./test-output/SequencerMac | xcbeautify
	@echo "Running Sequencer tests"
	xcodebuild -project Sequencer.xcodeproj -scheme "Sequencer" test -resultBundlePath ./test-output/Sequencer | xcbeautify

FORCE:
