format: FORCE
	./scripts/format.sh
	python3 ./scripts/add_license_preamble.py

build: FORCE
	cd ../looper-processor && make
	xcodebuild -project Sequencer.xcodeproj -scheme "Sequencer Mac" -configuration Release

test: FORCE
	./scripts/run-tests.sh

FORCE:
