import 'package:metronome/modules/history/history_controller.dart';
import 'package:metronome/modules/state/tap_tempo_controller.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../../bridge_generated.dart';
import 'metronome_state_model.dart';

class PreferenceKey {
  static const String tempo = "tempo";
  static const String volume = "volume";
  static const String beatsPerBar = "beatsPerBar";
}

class MetronomeStateController {
  final MetronomeStateModel model;
  final Metronome metronome;
  final HistoryStartStopHandler historyController;
  late final TapTempoController tapTempoController;

  MetronomeStateController(this.model, this.metronome, this.historyController) {
    tapTempoController = TapTempoController(this);
  }

  void setup() {
    metronome.getPlayhead().forEach((element) {
      model.setPlayhead(element);
    });

    SharedPreferences.getInstance().then((sharedPreferences) {
      var tempo = sharedPreferences.getDouble(PreferenceKey.tempo) ?? 120.0;
      var volume = sharedPreferences.getDouble(PreferenceKey.volume) ?? 0.3;
      var beatsPerBar =
          sharedPreferences.getInt(PreferenceKey.beatsPerBar) ?? 4;
      setTempo(tempo);
      setVolume(volume);
      setBeatsPerBar(beatsPerBar);
    });
  }

  void setTempo(double value) {
    metronome.setTempo(value: value);
    model.setTempo(value);

    SharedPreferences.getInstance().then((sharedPreferences) async {
      await sharedPreferences.setDouble(PreferenceKey.tempo, value);
    });
  }

  void setVolume(double value) {
    metronome.setVolume(value: value);
    model.setVolume(value);

    SharedPreferences.getInstance().then((sharedPreferences) async {
      await sharedPreferences.setDouble(PreferenceKey.volume, value);
    });
  }

  void setIsPlaying(bool value) {
    metronome.setIsPlaying(value: value);
    model.setIsPlaying(value);

    if (value) {
      historyController.onStart();
    } else {
      historyController.onEnd();
    }
  }

  void toggleIsPlaying() {
    var isPlaying = !model.isPlaying;
    setIsPlaying(isPlaying);
  }

  void setBeatsPerBar(int value) {
    model.setBeatsPerBar(value);
    metronome.setBeatsPerBar(value: value);

    SharedPreferences.getInstance().then((sharedPreferences) async {
      await sharedPreferences.setInt(PreferenceKey.beatsPerBar, value);
    });
  }

  void setSound(MetronomeSoundTypeTag sound) {
    model.setSound(sound);
    metronome.setSound(value: sound);
  }
}
