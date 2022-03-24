build: FORCE
	rm -rf ./public/*
	bash build-headers.sh
	cargo lipo --release --package looper-processor --targets=aarch64-apple-darwin
	cargo lipo --release --package looper-processor --targets=x86_64-apple-darwin
	cargo lipo --release --package looper-processor --targets=aarch64-apple-ios-sim
	cargo lipo --release --package looper-processor --targets=aarch64-apple-ios
	mkdir -p public/lib
	mkdir -p public/lib/macos-x86_64
	mkdir -p public/lib/macos-aarch64
	mkdir -p public/lib/ios-aarch64
	mkdir -p public/lib/ios-aarch64-sim
	mkdir -p public/include
	cp ../../../../target/x86_64-apple-darwin/release/liblooper_processor.a ./public/lib/macos-x86_64/liblooperengine.a
	cp ../../../../target/aarch64-apple-darwin/release/liblooper_processor.a ./public/lib/macos-aarch64/liblooperengine.a
	cp ../../../../target/aarch64-apple-ios/release/liblooper_processor.a ./public/lib/ios-aarch64/liblooperengine.a
	cp ../../../../target/aarch64-apple-ios-sim/release/liblooper_processor.a ./public/lib/ios-aarch64-sim/liblooperengine.a
	mkdir -p ./public/lib/macos/
	lipo -create ./public/lib/macos-x86_64/liblooperengine.a ./public/lib/macos-aarch64/liblooperengine.a -output ./public/lib/macos/liblooperengine.a

FORCE:
