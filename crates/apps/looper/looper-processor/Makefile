build: FORCE
	cargo build --release
	# cargo lipo --release
	mkdir -p public/lib
	mkdir -p public/include
	cp ../../../../target/release/liblooper_processor.a ./public/lib/liblooperengine.a

FORCE:
