build: FORCE
	cargo build --release
	mkdir -p ./src/Generated/swift
	uniffi-bindgen generate src/looper.udl --language swift --out-dir ./src/Generated/swift

FORCE:
