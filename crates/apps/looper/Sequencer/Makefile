format: FORCE
	./scripts/format.sh
	python3 ./scripts/add_license_preamble.py

clean:
	xcodebuild -project Sequencer.xcodeproj clean

build-rust-lib: FORCE
	cd ../looper-processor && make

build: FORCE
	make build-rust-lib
	xcodebuild -project Sequencer.xcodeproj -scheme "Sequencer Mac" -configuration Release

test: FORCE
	./scripts/run-tests.sh

FORCE:
