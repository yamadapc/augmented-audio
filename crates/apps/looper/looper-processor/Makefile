build: FORCE
	rm -rf ./public/*
	make build-headers
	make build-macos
	make build-ios

build-headers: FORCE
	bash build-headers.sh
	mkdir -p public/include

build-ios: FORCE
	cargo lipo --release --package looper-processor --targets=aarch64-apple-ios-sim
	cargo lipo --release --package looper-processor --targets=aarch64-apple-ios
	mkdir -p public/lib/ios-aarch64
	mkdir -p public/lib/ios-aarch64-sim
	cp ../../../../target/aarch64-apple-ios/release/liblooper_processor.a ./public/lib/ios-aarch64/liblooperengine.a
	cp ../../../../target/aarch64-apple-ios-sim/release/liblooper_processor.a ./public/lib/ios-aarch64-sim/liblooperengine.a

build-macos: FORCE
	cargo lipo --release --package looper-processor --targets=aarch64-apple-darwin
	cargo lipo --release --package looper-processor --targets=x86_64-apple-darwin
	mkdir -p public/lib/macos-x86_64
	mkdir -p public/lib/macos-aarch64
	cp ../../../../target/x86_64-apple-darwin/release/liblooper_processor.a ./public/lib/macos-x86_64/liblooperengine.a
	cp ../../../../target/aarch64-apple-darwin/release/liblooper_processor.a ./public/lib/macos-aarch64/liblooperengine.a
	mkdir -p ./public/lib/macos/
	lipo -create ./public/lib/macos-x86_64/liblooperengine.a ./public/lib/macos-aarch64/liblooperengine.a -output ./public/lib/macos/liblooperengine.a

generate-about: FORCE
	cargo about generate --manifest-path ./Cargo.toml about.hbs --output-file ./license.html

FORCE:
