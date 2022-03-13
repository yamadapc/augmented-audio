build: FORCE
	rm -r ./public/*
	bash build-headers.sh
	cargo build --release
	cargo lipo --release --package looper-processor --targets=aarch64-apple-ios-sim
	cargo lipo --release --package looper-processor --targets=aarch64-apple-ios
	mkdir -p public/lib
	mkdir -p public/lib/macos-aarch64
	mkdir -p public/lib/ios-aarch64
	mkdir -p public/lib/ios-aarch64-sim
	mkdir -p public/include
	cp ../../../../target/release/liblooper_processor.a ./public/lib/macos-aarch64/liblooperengine.a
	cp ../../../../target/aarch64-apple-ios/release/liblooper_processor.a ./public/lib/ios-aarch64/liblooperengine.a
	cp ../../../../target/aarch64-apple-ios-sim/release/liblooper_processor.a ./public/lib/ios-aarch64-sim/liblooperengine.a

FORCE:
