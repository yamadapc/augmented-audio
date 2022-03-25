format: FORCE
	swiftformat ./Sequencer/**/*.swift
	swiftformat ./SequencerUITests/**/*.swift
	swiftformat ./Sequencer\ AudioUnit/**/*.swift
	swiftformat ./SequencerUI/**/*.swift
	swiftformat ./Sequencer\ Mac/**/*.swift
	swiftformat ./Sequencer\ MacUITests/**/*.swift
	swiftformat ./SequencerEngine/**/*.swift

test: FORCE
	@echo "Checking if xcbeautify is installed"
	$(`which xcbeautify` || brew install xcbeautify)
	@echo "Running tests"
	xcodebuild -project Sequencer.xcodeproj -scheme "Sequencer Mac" test | xcbeautify

FORCE:
