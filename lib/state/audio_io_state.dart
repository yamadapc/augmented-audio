import 'package:flutter/widgets.dart';
import 'package:mobx/mobx.dart';

part 'audio_io_state.g.dart';

class AudioIOState = _AudioIOState with _$AudioIOState;

abstract class _AudioIOState with Store {
  @observable
  ObservableList<AudioInput> availableInputs = ObservableList.of([]);

  _AudioIOState(this.availableInputs);
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
  }) : super(child: child);

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
