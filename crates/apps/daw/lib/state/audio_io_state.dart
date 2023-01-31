// ignore_for_file: library_private_types_in_public_api

import 'package:flutter/widgets.dart';
import 'package:flutter_daw_mock_ui/state/entity.dart';
import 'package:mobx/mobx.dart';

part 'audio_io_state.g.dart';

class AudioIOState extends _AudioIOState with _$AudioIOState {
  @override
  ActionController get _$_AudioIOStateActionController => getActionController();
}

abstract class _AudioIOState with Store, Entity {
  @override
  String id = "/AudioIOState";

  @observable
  AudioDevice? currentInputDevice;

  @observable
  AudioDevice? currentOutputDevice;

  @observable
  ObservableList<AudioDevice> inputDevices = ObservableList.of([]);

  @observable
  ObservableList<AudioDevice> outputDevices = ObservableList.of([]);

  @observable
  ObservableList<AudioInput> availableInputs = ObservableList.of([]);

  @action
  void setInputDevice(AudioDevice? inputDevice) {
    currentInputDevice = inputDevice;
  }

  @action
  void setOutputDevice(AudioDevice? outputDevice) {
    currentOutputDevice = outputDevice;
  }
}

class AudioDevice {
  String id = "";
  String title = "";

  AudioDevice({required this.title}) {
    id = title.hashCode.toString();
  }

  @override
  String toString() {
    return 'AudioDevice:$id';
  }
}

class AudioInput = _AudioInput with _$AudioInput;

abstract class _AudioInput with Store {
  @observable
  String id = "";

  @observable
  String title = "";

  _AudioInput(this.id, this.title);
}

class AudioIOStateProvider extends InheritedWidget {
  final AudioIOState audioIOState;

  const AudioIOStateProvider({
    Key? key,
    required this.audioIOState,
    required Widget child,
  }) : super(key: key, child: child);

  static AudioIOState stateOf(BuildContext context) {
    AudioIOStateProvider audioIOStateProvider = context
        .getElementForInheritedWidgetOfExactType<AudioIOStateProvider>()
        ?.widget as AudioIOStateProvider;
    return audioIOStateProvider.audioIOState;
  }

  @override
  bool updateShouldNotify(AudioIOStateProvider oldWidget) =>
      oldWidget.audioIOState != audioIOState;
}
