import '../../bridge_generated.dart';
import 'metronome_state_model.dart';

class MetronomeStateController {
  final MetronomeStateModel model;
  final Metronome metronome;

  MetronomeStateController(this.model, this.metronome);

  void setup() {
    metronome.getPlayhead().forEach((element) {
      model.setPlayhead(element);
    });
  }

  void setTempo(double value) {
    metronome.setTempo(value: value);
    model.setTempo(value);
  }

  void setVolume(double value) {
    metronome.setVolume(value: value);
    model.setVolume(value);
  }

  void setIsPlaying(bool value) {
    metronome.setIsPlaying(value: value);
    model.setIsPlaying(value);
  }

  void toggleIsPlaying() {
    setIsPlaying(!model.isPlaying);
  }
}
