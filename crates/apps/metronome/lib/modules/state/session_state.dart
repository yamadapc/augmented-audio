import 'dart:async';

import 'package:clock/clock.dart';
import 'package:mobx/mobx.dart';

part 'session_state.g.dart';

class SessionState = _SessionState with _$SessionState;

abstract class _SessionState with Store {
  @observable
  bool isPlaying = false;

  @observable
  DateTime? start;

  @observable
  DateTime now = clock.now();

  @observable
  Timer? timer;

  Duration get duration {
    if (start == null) {
      return Duration.zero;
    }
    return now.difference(start!);
  }

  @action
  void startSession() {
    isPlaying = true;
    start = clock.now();

    timer = Timer.periodic(const Duration(milliseconds: 500), (timer) {
      runInAction(() {
        now = clock.now();
      });
    });
  }

  @action
  void stopSession() {
    isPlaying = false;
    start = null;
    timer?.cancel();
    timer = null;
  }
}
