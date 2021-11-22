import 'dart:ffi';

import 'package:flutter_daw_mock_ui/bridge_generated.dart';

FlutterDawMockUi initialize() {
  var dylib = DynamicLibrary.open("../../../target/debug/libdaw_ui.dylib")
  return FlutterDawMockUi(dylib);
}
