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
	./scripts/run-tests.sh

FORCE:
