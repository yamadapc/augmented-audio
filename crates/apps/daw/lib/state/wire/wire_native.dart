import 'dart:ffi';

import 'package:flutter_daw_mock_ui/bridge_generated.dart';
import 'package:flutter_daw_mock_ui/state/wire/wire_base.dart';

DawUi? dawUi;

DawUi? initialize() {
  if (dawUi != null) {
    return dawUi!;
  }

  DynamicLibrary dylib = DynamicLibrary.open(
      "/Users/yamadapc/projects/rust-audio-software/target/debug/libdaw_ui.dylib");
  dawUi = DawUi(dylib);
  return dawUi!;
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
  return NativeAudioIOStore(initialize()!);
}
