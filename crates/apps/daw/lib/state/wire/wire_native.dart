import 'dart:developer';
import 'dart:ffi';

import 'package:flutter/foundation.dart';
import 'package:flutter_daw_mock_ui/bridge_generated.dart';
import 'package:flutter_daw_mock_ui/state/wire/wire_base.dart';

DawUi? dawUi;

DawUi? initialize() {
  if (dawUi != null) {
    return dawUi!;
  }

  try {
    DynamicLibrary dylib = DynamicLibrary.open(
        "/Users/yamadapc/projects/rust-audio-software/target/debug/libdaw_ui.dylib");
    dawUi = DawUi(dylib);
    return dawUi!;
  } catch (err) {
    if (kDebugMode) {
      log("Failed to initialize native library.");
      return null;
    } else {
      rethrow;
    }
  }
}

class NativeAudioIOStore with AudioIOStore {
  final DawUi api;
  NativeAudioIOStore(this.api);

  @override
  Future<String> getInputDevices() {
    return api.audioIoGetInputDevices();
  }
}

AudioIOStore? getAudioIOStore() {
  var api = initialize();
  if (api != null) {
    return NativeAudioIOStore(api);
  } else {
    return null;
  }
}
