// ignore_for_file: use_setters_to_change_properties, avoid_positional_boolean_parameters

import 'package:metronome/modules/state/session_state.dart';
import 'package:metronome/src/rust/internal/processor.dart';
import 'package:mobx/mobx.dart';

part 'metronome_state_model.g.dart';

// ignore: library_private_types_in_public_api
class MetronomeStateModel = _MetronomeStateModel with _$MetronomeStateModel;

abstract class _MetronomeStateModel with Store {
  @observable
  bool isPlaying = false;

  @observable
  double volume = 0.3;

  @observable
  double tempo = 120.0;

  @observable
  double playhead = 0.0;

  @observable
  int beatsPerBar = 4;

  @observable
  MetronomeSoundTypeTag sound = MetronomeSoundTypeTag.sine;

  @observable
  SessionState sessionState = SessionState();

  @action
  void setPlayhead(double value) {
    if (playhead == value) {
      return;
    }

    playhead = value;
  }

  @action
  void setTempo(double value) {
    tempo = value;
  }

  @action
  void setIsPlaying(bool value) {
    isPlaying = value;
  }

  @action
  void setVolume(double value) {
    volume = value;
  }

  @action
  void setBeatsPerBar(int value) {
    beatsPerBar = value;
  }

  @action
  void setSound(MetronomeSoundTypeTag value) {
    sound = value;
  }
}
