# daw

This is a flutter front-end for a Rust audio application.

⚠️⚠️ **This sub-directory is licensed under GPLv3. Please see LICENSE for more information** ⚠️⚠️

## Architecture

Flutter uses dart as a programming language. Rust is compiled into dynamic/static libraries which dart will interact
with through FFI via message passing.

The rust side mostly runs an actor system, with different components being implemented as objects with mail-boxes. This
simplifies multi-threading.

Bindings will be auto-generated where possible.

Dart code uses `mobx` for state management.

## Codegen

mobx / JSON

```
flutter packages pub run build_runner watch
```

Rust bridge

```
flutter_rust_bridge_codegen --rust-input src/api.rs --dart-output lib/bridge_generated.dart
```