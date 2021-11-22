import 'dart:convert';

import 'package:flutter_daw_mock_ui/bridge_generated.dart';
import 'package:flutter_daw_mock_ui/state/audio_io_state.dart';
import 'package:json_annotation/json_annotation.dart';
import 'package:mobx/mobx.dart';

part 'audio_io_service.g.dart';

@JsonSerializable()
class RemoteAudioDevice {
  final String name;
  final List<int> sampleRateRange;
  final List<int>? bufferSizeRange;

  RemoteAudioDevice({required this.name,
    required this.sampleRateRange,
    required this.bufferSizeRange});

  factory RemoteAudioDevice.fromJson(Map<String, dynamic> json) =>
      _$RemoteAudioDeviceFromJson(json);
}

@JsonSerializable()
class RemoteDevicesList {
  final List<RemoteAudioDevice> inputDevices;
  final List<RemoteAudioDevice> outputDevices;

  RemoteDevicesList({required this.inputDevices, required this.outputDevices});

  factory RemoteDevicesList.fromJson(Map<String, dynamic> json) =>
      _$RemoteDevicesListFromJson(json);
}

class AudioIOService {
  final DawUi api;
  final AudioIOState audioIOState;

  AudioIOService(this.api, this.audioIOState);

  /// Fetch audio devices from Rust & push them into the flutter state.
  Future<void> syncDevices() async {
    var devicesListStr = await api.audioIoGetInputDevices();
    var remoteDevicesList =
    RemoteDevicesList.fromJson(json.decode(devicesListStr));

    runInAction(() {
      audioIOState.inputDevices.clear();
      audioIOState.outputDevices.clear();

      for (var inputDevice in remoteDevicesList.inputDevices) {
        audioIOState.inputDevices.add(AudioDevice(
          title: inputDevice.name,
        ));
      }

      for (var outputDevice in remoteDevicesList.outputDevices) {
        audioIOState.outputDevices.add(AudioDevice(
          title: outputDevice.name,
        ));
      }
    });
  }
}
