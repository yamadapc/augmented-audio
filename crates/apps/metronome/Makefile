build_and_test: FORCE
	make build-libs
	make build-libs-ios
	make build-bindings
	cargo test

	make build-dart-codegen
	flutter test
	dart format --fix ./lib
	dart format --fix ./test

lint:
	flutter analyze

build-dart-codegen: FORCE
	flutter packages pub run build_runner build --delete-conflicting-outputs

watch-dart-codegen: FORCE
	flutter packages pub run build_runner watch

build-bindings: FORCE
	cp ios/Runner/bridge_generated.h macos/Runner/bridge_generated.h
	sed -i '' '/jint JNI_OnLoad(JavaVM vm, void \*res);/d' ios/Runner/bridge_generated.h macos/Runner/bridge_generated.h

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
	mkdir -p public/lib/-iphonesimulator
	mkdir -p public/lib/-iphoneos
	cp ../../../target/aarch64-apple-ios/release/libmetronome.a ./public/lib/-iphoneos/libmetronome.a
	cp ../../../target/aarch64-apple-ios-sim/release/libmetronome.a ./public/lib/-iphonesimulator/libmetronome.a

build-test-android: FORCE
	cargo ndk -t armeabi-v7a -t arm64-v8a -o android/app/src/main/jniLibs build

setup-ndk:
	cp ${NDK_HOME}/toolchains/llvm/prebuilt/darwin-x86_64/sysroot/usr/lib/aarch64-linux-android/libc++_shared.so ./android/app/src/main/jniLibs/arm64-v8a/
	cp ${NDK_HOME}/toolchains/llvm/prebuilt/darwin-x86_64/sysroot/usr/lib/arm-linux-androideabi/libc++_shared.so ./android/app/src/main/jniLibs/armeabi-v7a/
	cp ${NDK_HOME}/toolchains/llvm/prebuilt/darwin-x86_64/sysroot/usr/lib/x86_64-linux-android/libc++_shared.so ./android/app/src/main/jniLibs/x86_64/
	cp ${NDK_HOME}/toolchains/llvm/prebuilt/darwin-x86_64/sysroot/usr/lib/i686-linux-android/libc++_shared.so ./android/app/src/main/jniLibs/x86/



FORCE:
