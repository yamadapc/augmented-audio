import 'package:firebase_analytics/firebase_analytics.dart';
import 'package:mobx/mobx.dart';

import '../../ui/utils/debounce.dart';
import 'metronome_state_controller.dart';

class TapTempoController {
  final ObservableList<int> presses = ObservableList.of([]);
  final MetronomeStateController stateController;
  late final Debounce _debounce;

  TapTempoController(this.stateController) {
    _debounce = Debounce(2000);
  }

  void onPress() {
    presses.add(DateTime.now().millisecondsSinceEpoch);

    if (presses.length >= 3) {
      flushPresses();
    }

    if (presses.length > 1) {
      final lastPress = presses[presses.length - 1];
      final secondLastPress = presses[presses.length - 2];
      _debounce.setDebounceMs((lastPress - secondLastPress) * 2);
      _debounce.run(() {
        flushPresses();
        _debounce.setDebounceMs(2000);
        presses.clear();

        var analytics = FirebaseAnalytics.instance;
        analytics.logEvent(name: "TapTempoController__flushed");
      });
    }
  }

  void flushPresses() {
    final last4Presses = presses.reversed.take(4).toList().reversed;
    var lastPress = last4Presses.first;
    final deltas = last4Presses.skip(1).map((timestamp) {
      final delta = timestamp - lastPress;
      lastPress = timestamp;
      return delta;
    });

    final msPerBeat =
        deltas.reduce((value, element) => value + element) / deltas.length;
    final secsPerBeat = msPerBeat / 1000;
    final beatsPerSec = 1 / secsPerBeat;
    final beatsPerMinute = beatsPerSec * 60;

    stateController.setTempo(beatsPerMinute);
  }
}
