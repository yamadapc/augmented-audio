build_and_test: FORCE
	make build-dart-codegen
	make build-bindings
	make build-libs
	cargo test
	flutter test
	dart format --fix .

build-dart-codegen: FORCE
	flutter packages pub run build_runner build

watch-dart-codegen: FORCE
	flutter packages pub run build_runner watch

build-bindings: FORCE
	flutter_rust_bridge_codegen --rust-input src/api.rs --dart-output lib/bridge_generated.dart --c-output ios/Runner/bridge_generated.h

build-libs: FORCE
	cargo lipo --release --package metronome
	cargo build --release

FORCE:
