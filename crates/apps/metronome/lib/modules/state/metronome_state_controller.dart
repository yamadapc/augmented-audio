// ignore_for_file: avoid_positional_boolean_parameters

import 'dart:async';

import 'package:metronome/modules/analytics/analytics.dart';
import 'package:metronome/modules/history/history_controller.dart';
import 'package:metronome/modules/state/metronome_state_model.dart';
import 'package:metronome/modules/state/tap_tempo_controller.dart';
import 'package:metronome/src/rust/api.dart';
import 'package:metronome/src/rust/frb_generated.dart';
import 'package:metronome/src/rust/internal/processor.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:wakelock_plus/wakelock_plus.dart';

class PreferenceKey {
  static const String tempo = "tempo";
  static const String volume = "volume";
  static const String beatsPerBar = "beatsPerBar";
}

class MetronomeStateController {
  final MetronomeStateModel model;
  final HistoryStartStopHandler historyController;
  late final TapTempoController tapTempoController;
  final Analytics analytics;

  Timer? timeout;

  MetronomeStateController(
    this.model,
    this.historyController,
    this.analytics,
  ) {
    tapTempoController = TapTempoController(this, analytics);
  }

  void setup() {
    timeout = Timer.periodic(
      const Duration(milliseconds: 100),
      (timer) async {
        final p = await getPlayhead();
        model.setPlayhead(p);
      },
    );

    SharedPreferences.getInstance().then((sharedPreferences) {
      final tempo = sharedPreferences.getDouble(PreferenceKey.tempo) ?? 120.0;
      final volume = sharedPreferences.getDouble(PreferenceKey.volume) ?? 0.3;
      final beatsPerBar =
          sharedPreferences.getInt(PreferenceKey.beatsPerBar) ?? 4;
      setTempo(tempo);
      setVolume(volume);
      setBeatsPerBar(beatsPerBar);
    });
  }

  void stop() {
    timeout?.cancel();
  }

  void setTempo(double value) {
    setTempo(value);
    model.setTempo(value);

    SharedPreferences.getInstance().then((sharedPreferences) async {
      await sharedPreferences.setDouble(PreferenceKey.tempo, value);
    });
  }

  void setVolume(double value) {
    setVolume(value);
    model.setVolume(value);

    SharedPreferences.getInstance().then((sharedPreferences) async {
      await sharedPreferences.setDouble(PreferenceKey.volume, value);
    });
  }

  void setIsPlaying(bool value) {
    setIsPlaying(value);
    model.setIsPlaying(value);

    if (value) {
      historyController.onStart();
      Wakelock.enable();
    } else {
      historyController.onEnd();
      Wakelock.disable();
    }
  }

  void toggleIsPlaying() {
    final isPlaying = !model.isPlaying;
    setIsPlaying(isPlaying);
  }

  void setBeatsPerBar(int value) {
    model.setBeatsPerBar(value);
    setBeatsPerBar(value);

    SharedPreferences.getInstance().then((sharedPreferences) async {
      await sharedPreferences.setInt(PreferenceKey.beatsPerBar, value);
    });
  }

  void setSound(MetronomeSoundTypeTag sound) {
    model.setSound(sound);
    setSound(sound);
  }

  void increaseTempo({double? increment}) {
    setTempo(model.tempo + (increment ?? 1));
  }

  void decreaseTempo({double? decrement}) {
    setTempo(model.tempo - (decrement ?? 1));
  }
}
