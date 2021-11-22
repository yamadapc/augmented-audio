import 'dart:ffi';

import 'package:flutter_daw_mock_ui/bridge_generated.dart';

DynamicLibrary dylib = DynamicLibrary.open(
    "/Users/yamadapc/projects/rust-audio-software/target/debug/libdaw_ui.dylib");

DawUi dawUi = DawUi(dylib);

DawUi initialize() {
  return dawUi;
}
