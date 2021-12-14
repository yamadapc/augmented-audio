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

class NativeAudioGraph with AudioGraph {
  DawUi api;

  NativeAudioGraph(this.api) {
    api.audioGraphSetup();
  }

  @override
  Future<int> connect({required int inputIndex, required int outputIndex}) {
    return api.audioGraphConnect(
        inputIndex: inputIndex, outputIndex: outputIndex);
  }

  @override
  Future<int> createNode({required String name}) {
    return api.audioNodeCreate(audioProcessorName: name);
  }

  @override
  Future<List<int>> systemIndexes() {
    return api.audioGraphGetSystemIndexes();
  }
}

AudioGraph? getAudioGraph() {
  var api = initialize();
  if (api != null) {
    return NativeAudioGraph(api);
  } else {
    return null;
  }
}

class NativeAudioThread with AudioThread {
  DawUi api;

  NativeAudioThread(this.api);

  @override
  Future<void> setOptions(
      {required String inputDeviceId, required String outputDeviceId}) {
    return api.audioThreadSetOptions(
        outputDeviceId: outputDeviceId, inputDeviceId: inputDeviceId);
  }
}

AudioThread? getAudioThread() {
  var api = initialize();
  if (api != null) {
    return NativeAudioThread(api);
  } else {
    return null;
  }
}
