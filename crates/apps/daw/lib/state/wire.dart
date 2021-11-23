import 'dart:ffi';

import 'package:flutter_daw_mock_ui/bridge_generated.dart';

DawUi? dawUi;

DawUi initialize() {
  if (dawUi != null) {
    return dawUi!;
  }

  DynamicLibrary dylib = DynamicLibrary.open(
      "/Users/yamadapc/projects/rust-audio-software/target/debug/libdaw_ui.dylib");
  dawUi = DawUi(dylib);
  return dawUi!;
}
