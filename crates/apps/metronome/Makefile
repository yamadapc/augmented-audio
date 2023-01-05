build_and_test: FORCE
	make build-bindings
	make build-libs
	make build-libs-ios
	cargo test

	make build-dart-codegen
	flutter test
	dart format --fix .

lint:
	flutter analyze

build-dart-codegen: FORCE
	flutter packages pub run build_runner build --delete-conflicting-outputs

watch-dart-codegen: FORCE
	flutter packages pub run build_runner watch

build-bindings: FORCE
	flutter_rust_bridge_codegen  \
		--rust-input src/api.rs \
		--dart-output lib/bridge_generated.dart \
		--c-output ios/Runner/bridge_generated.h # \
		# --llvm-path /usr/local/opt/llvm
	cp ios/Runner/bridge_generated.h macos/Runner/bridge_generated.h

build-libs: FORCE
	cargo lipo --release --package metronome --targets=x86_64-apple-darwin
	cargo lipo --release --package metronome --targets=aarch64-apple-darwin
	mkdir -p ./public/lib/macos/
	mkdir -p ./public/lib/macos-x86_64/
	mkdir -p ./public/lib/macos-aarch64/
	cp ../../../target/x86_64-apple-darwin/release/libmetronome.a ./public/lib/macos-x86_64/libmetronome.a
	cp ../../../target/aarch64-apple-darwin/release/libmetronome.a ./public/lib/macos-aarch64/libmetronome.a
	lipo -create ./public/lib/macos-x86_64/libmetronome.a ./public/lib/macos-aarch64/libmetronome.a -output ./public/lib/macos/libmetronome.a

build-libs-ios: FORCE
	cargo lipo --release --package metronome --targets=aarch64-apple-ios
	cargo lipo --release --package metronome --targets=aarch64-apple-ios-sim
	mkdir -p public/lib/ios-aarch64
	mkdir -p public/lib/ios-aarch64-sim
	mkdir -p public/lib/ios
	cp ../../../target/aarch64-apple-ios/release/libmetronome.a ./public/lib/ios-aarch64/libmetronome.a
	cp ../../../target/aarch64-apple-ios-sim/release/libmetronome.a ./public/lib/ios-aarch64-sim/libmetronome.a

FORCE:
