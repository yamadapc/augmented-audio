import 'package:metronome/modules/history/history_controller.dart';
import 'package:shared_preferences/shared_preferences.dart';

import '../../bridge_generated.dart';
import 'metronome_state_model.dart';

class MetronomeStateController {
  final MetronomeStateModel model;
  final Metronome metronome;
  final HistoryStartStopHandler historyController;

  MetronomeStateController(this.model, this.metronome, this.historyController);

  void setup() {
    metronome.getPlayhead().forEach((element) {
      model.setPlayhead(element);
    });

    SharedPreferences.getInstance().then((sharedPreferences) {
      var tempo = sharedPreferences.getDouble("tempo") ?? 120.0;
      var volume = sharedPreferences.getDouble("volume") ?? 0.3;
      setTempo(tempo);
      setVolume(volume);
    });
  }

  void setTempo(double value) {
    metronome.setTempo(value: value);
    model.setTempo(value);

    SharedPreferences.getInstance().then((sharedPreferences) async {
      await sharedPreferences.setDouble("tempo", value);
    });
  }

  void setVolume(double value) {
    metronome.setVolume(value: value);
    model.setVolume(value);

    SharedPreferences.getInstance().then((sharedPreferences) async {
      await sharedPreferences.setDouble("volume", value);
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
}
