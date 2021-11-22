import 'package:flutter_daw_mock_ui/state/entity.dart';
import 'package:flutter_daw_mock_ui/ui/main_content_layout/tracks_view/track_view/track_controls/knob_field.dart';
import 'package:graphx/graphx.dart';
import 'package:mobx/mobx.dart';

part 'project.g.dart';

class Project = _Project with _$Project;

abstract class _Project with Store, Entity {
  @override
  String id = "";

  @observable
  String title = "Untitled";

  @observable
  TracksList tracksList = TracksList();
}

class TracksList extends _TracksList with _$TracksList {
  @override
  ActionController get _$_TracksListActionController => getActionController();
}

abstract class _TracksList with Store, Entity {
  @override
  String id = "TracksList";

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
    this.id = "/Track:$id";
    this.title = title;
    this.clips = clips ?? ObservableList.of([]);
    this.parent = parent;
    pan = DoubleValue(this.id + "/pan");
    sends = ObservableList.of([DoubleValue(this.id + "/sends/A")]);
  }

  @override
  ActionController get _$_TrackActionController => getActionController();

  void select() {
    parent?.selectTrack(this);
  }
}

abstract class _Track with Store, Entity {
  @override
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
  DoubleValue pan = DoubleValue("pan");

  @observable
  ObservableList<DoubleValue> sends = ObservableList.of([
    DoubleValue("sends/A"),
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

class DoubleValue extends _DoubleValue with _$DoubleValue {
  DoubleValue(String id) {
    this.id = id;
  }

  @override
  ActionController get _$_DoubleValueActionController => getActionController();
}

abstract class _DoubleValue with Store, KnobFieldModel, Entity {
  @override
  String id = "";

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
