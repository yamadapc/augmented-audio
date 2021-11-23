build_and_test: FORCE
	make build-bindings
	cargo build
	cargo test
	flutter test
	dart format

build-bindings: FORCE
	flutter_rust_bridge_codegen --rust-input src/api.rs --dart-output lib/bridge_generated.dart

FORCE:
