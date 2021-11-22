import 'package:flutter_daw_mock_ui/ui/main_content_layout/tracks_view/track_view/track_controls/knob_field.dart';
import 'package:graphx/graphx.dart';
import 'package:mobx/mobx.dart';

part 'project.g.dart';

class Project = _Project with _$Project;

abstract class _Project with Store {
  @observable
  String title = "Untitled";

  @observable
  TracksList tracksList = TracksList();
}

class TracksList = _TracksList with _$TracksList;

abstract class _TracksList with Store {
  @observable
  ObservableList<Track> tracks = ObservableList.of([]);

  @observable
  Track? selectedTrack;

  @action
  void reorderTracks(int sourceIndex, int targetIndex) {
    var elem = tracks[sourceIndex];
    tracks.removeAt(sourceIndex);
    var targetPrime =
        Math.max(sourceIndex < targetIndex ? targetIndex - 1 : targetIndex, 0);
    tracks.insert(targetPrime, elem);
  }

  @action
  void selectTrack(Track track) {
    selectedTrack = track;
  }
}

class Track extends _Track with _$Track {
  Track(
      {required String id, required String title, clips, TracksList? parent}) {
    this.id = id;
    this.title = title;
    this.clips = clips ?? ObservableList.of([]);
    this.parent = parent;
  }

  void select() {
    parent?.selectTrack(this);
  }
}

abstract class _Track with Store {
  @observable
  String id = "";

  @observable
  String title = "";

  @observable
  String audioInputId = "none";

  @observable
  ObservableList<Clip> clips = ObservableList.of([]);

  @observable
  ObservableList<AudioEffectInstance> audioEffects = ObservableList.of([]);

  @observable
  DoubleValue pan = DoubleValue();

  @observable
  ObservableList<DoubleValue> sends = ObservableList.of([
    DoubleValue(),
  ]);

  @computed
  bool get isSelected {
    return parent?.selectedTrack == this;
  }

  TracksList? parent;

  @action
  void setAudioInputId(String audioInputId) {
    this.audioInputId = audioInputId;
  }
}

class DoubleValue = _DoubleValue with _$DoubleValue;

abstract class _DoubleValue with Store, KnobFieldModel {
  @observable
  double value = 0.0;

  @override
  double getValue() {
    return value;
  }

  @override
  @action
  void setValue(double value) {
    this.value = value;
  }
}

class AudioEffectInstance = _AudioEffectInstance with _$AudioEffectInstance;

abstract class _AudioEffectInstance with Store {
  @observable
  String id = "";

  @observable
  String title = "";

  @observable
  String effectTypeId = "";
}

class Clip = _Clip with _$Clip;

abstract class _Clip with Store {
  @observable
  String id = "";

  @observable
  String title = "";

  _Clip({required this.title});
}
