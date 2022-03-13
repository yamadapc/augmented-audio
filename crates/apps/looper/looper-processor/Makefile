build: FORCE
	cargo build --release
	cargo lipo --release
	mkdir -p public/lib
	mkdir -p public/include
	cp ../../../../target/release/liblooper-processor.dylib ./public/lib/liblooperengine.dylib
