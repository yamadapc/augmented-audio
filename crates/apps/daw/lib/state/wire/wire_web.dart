import 'package:flutter_daw_mock_ui/state/wire/wire_base.dart';

abstract class DawUi {
  Future<int> initializeLogger();
  Future<int> initializeAudio();
  Future<int> startPlayback();
  Future<int> stopPlayback();
  Future<int> setVstFilePath({required String path});
  Future<int> setInputFilePath({required String path});
  Future<String> audioIoGetInputDevices();
  Stream<String> getEventsSink();
}

DawUi? dawUi;

DawUi? initialize() {
  return null;
}

AudioIOStore? getAudioIOStore() {
  return null;
}

AudioGraph? getAudioGraph() {
  return null;
}

AudioThread? getAudioThread() {
  return null;
}
