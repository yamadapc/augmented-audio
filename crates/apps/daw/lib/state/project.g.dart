// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'project.dart';

// **************************************************************************
// StoreGenerator
// **************************************************************************

// ignore_for_file: non_constant_identifier_names, unnecessary_brace_in_string_interps, unnecessary_lambdas, prefer_expression_function_bodies, lines_longer_than_80_chars, avoid_as, avoid_annotating_with_dynamic

mixin _$Project on _Project, Store {
  final _$titleAtom = Atom(name: '_Project.title');

  @override
  String get title {
    _$titleAtom.reportRead();
    return super.title;
  }

  @override
  set title(String value) {
    _$titleAtom.reportWrite(value, super.title, () {
      super.title = value;
    });
  }

  final _$tracksListAtom = Atom(name: '_Project.tracksList');

  @override
  TracksList get tracksList {
    _$tracksListAtom.reportRead();
    return super.tracksList;
  }

  @override
  set tracksList(TracksList value) {
    _$tracksListAtom.reportWrite(value, super.tracksList, () {
      super.tracksList = value;
    });
  }

  @override
  String toString() {
    return '''
title: ${title},
tracksList: ${tracksList}
    ''';
  }
}

mixin _$TracksList on _TracksList, Store {
  final _$tracksAtom = Atom(name: '_TracksList.tracks');

  @override
  ObservableList<Track> get tracks {
    _$tracksAtom.reportRead();
    return super.tracks;
  }

  @override
  set tracks(ObservableList<Track> value) {
    _$tracksAtom.reportWrite(value, super.tracks, () {
      super.tracks = value;
    });
  }

  final _$selectedTrackAtom = Atom(name: '_TracksList.selectedTrack');

  @override
  Track? get selectedTrack {
    _$selectedTrackAtom.reportRead();
    return super.selectedTrack;
  }

  @override
  set selectedTrack(Track? value) {
    _$selectedTrackAtom.reportWrite(value, super.selectedTrack, () {
      super.selectedTrack = value;
    });
  }

  final _$_TracksListActionController = ActionController(name: '_TracksList');

  @override
  void reorderTracks(int sourceIndex, int targetIndex) {
    final _$actionInfo = _$_TracksListActionController.startAction(
        name: '_TracksList.reorderTracks');
    try {
      return super.reorderTracks(sourceIndex, targetIndex);
    } finally {
      _$_TracksListActionController.endAction(_$actionInfo);
    }
  }

  @override
  void selectTrack(Track track) {
    final _$actionInfo = _$_TracksListActionController.startAction(
        name: '_TracksList.selectTrack');
    try {
      return super.selectTrack(track);
    } finally {
      _$_TracksListActionController.endAction(_$actionInfo);
    }
  }

  @override
  String toString() {
    return '''
tracks: ${tracks},
selectedTrack: ${selectedTrack}
    ''';
  }
}

mixin _$Track on _Track, Store {
  Computed<bool>? _$isSelectedComputed;

  @override
  bool get isSelected => (_$isSelectedComputed ??=
          Computed<bool>(() => super.isSelected, name: '_Track.isSelected'))
      .value;

  final _$idAtom = Atom(name: '_Track.id');

  @override
  String get id {
    _$idAtom.reportRead();
    return super.id;
  }

  @override
  set id(String value) {
    _$idAtom.reportWrite(value, super.id, () {
      super.id = value;
    });
  }

  final _$titleAtom = Atom(name: '_Track.title');

  @override
  String get title {
    _$titleAtom.reportRead();
    return super.title;
  }

  @override
  set title(String value) {
    _$titleAtom.reportWrite(value, super.title, () {
      super.title = value;
    });
  }

  final _$audioInputIdAtom = Atom(name: '_Track.audioInputId');

  @override
  String get audioInputId {
    _$audioInputIdAtom.reportRead();
    return super.audioInputId;
  }

  @override
  set audioInputId(String value) {
    _$audioInputIdAtom.reportWrite(value, super.audioInputId, () {
      super.audioInputId = value;
    });
  }

  final _$clipsAtom = Atom(name: '_Track.clips');

  @override
  ObservableList<Clip> get clips {
    _$clipsAtom.reportRead();
    return super.clips;
  }

  @override
  set clips(ObservableList<Clip> value) {
    _$clipsAtom.reportWrite(value, super.clips, () {
      super.clips = value;
    });
  }

  final _$audioEffectsAtom = Atom(name: '_Track.audioEffects');

  @override
  ObservableList<AudioEffectInstance> get audioEffects {
    _$audioEffectsAtom.reportRead();
    return super.audioEffects;
  }

  @override
  set audioEffects(ObservableList<AudioEffectInstance> value) {
    _$audioEffectsAtom.reportWrite(value, super.audioEffects, () {
      super.audioEffects = value;
    });
  }

  final _$panAtom = Atom(name: '_Track.pan');

  @override
  DoubleValue get pan {
    _$panAtom.reportRead();
    return super.pan;
  }

  @override
  set pan(DoubleValue value) {
    _$panAtom.reportWrite(value, super.pan, () {
      super.pan = value;
    });
  }

  final _$sendsAtom = Atom(name: '_Track.sends');

  @override
  ObservableList<DoubleValue> get sends {
    _$sendsAtom.reportRead();
    return super.sends;
  }

  @override
  set sends(ObservableList<DoubleValue> value) {
    _$sendsAtom.reportWrite(value, super.sends, () {
      super.sends = value;
    });
  }

  final _$_TrackActionController = ActionController(name: '_Track');

  @override
  void setAudioInputId(String audioInputId) {
    final _$actionInfo =
        _$_TrackActionController.startAction(name: '_Track.setAudioInputId');
    try {
      return super.setAudioInputId(audioInputId);
    } finally {
      _$_TrackActionController.endAction(_$actionInfo);
    }
  }

  @override
  String toString() {
    return '''
id: ${id},
title: ${title},
audioInputId: ${audioInputId},
clips: ${clips},
audioEffects: ${audioEffects},
pan: ${pan},
sends: ${sends},
isSelected: ${isSelected}
    ''';
  }
}

mixin _$DoubleValue on _DoubleValue, Store {
  final _$valueAtom = Atom(name: '_DoubleValue.value');

  @override
  double get value {
    _$valueAtom.reportRead();
    return super.value;
  }

  @override
  set value(double value) {
    _$valueAtom.reportWrite(value, super.value, () {
      super.value = value;
    });
  }

  final _$_DoubleValueActionController = ActionController(name: '_DoubleValue');

  @override
  void setValue(double value) {
    final _$actionInfo = _$_DoubleValueActionController.startAction(
        name: '_DoubleValue.setValue');
    try {
      return super.setValue(value);
    } finally {
      _$_DoubleValueActionController.endAction(_$actionInfo);
    }
  }

  @override
  String toString() {
    return '''
value: ${value}
    ''';
  }
}

mixin _$AudioEffectInstance on _AudioEffectInstance, Store {
  final _$idAtom = Atom(name: '_AudioEffectInstance.id');

  @override
  String get id {
    _$idAtom.reportRead();
    return super.id;
  }

  @override
  set id(String value) {
    _$idAtom.reportWrite(value, super.id, () {
      super.id = value;
    });
  }

  final _$titleAtom = Atom(name: '_AudioEffectInstance.title');

  @override
  String get title {
    _$titleAtom.reportRead();
    return super.title;
  }

  @override
  set title(String value) {
    _$titleAtom.reportWrite(value, super.title, () {
      super.title = value;
    });
  }

  final _$effectTypeIdAtom = Atom(name: '_AudioEffectInstance.effectTypeId');

  @override
  String get effectTypeId {
    _$effectTypeIdAtom.reportRead();
    return super.effectTypeId;
  }

  @override
  set effectTypeId(String value) {
    _$effectTypeIdAtom.reportWrite(value, super.effectTypeId, () {
      super.effectTypeId = value;
    });
  }

  @override
  String toString() {
    return '''
id: ${id},
title: ${title},
effectTypeId: ${effectTypeId}
    ''';
  }
}

mixin _$Clip on _Clip, Store {
  final _$idAtom = Atom(name: '_Clip.id');

  @override
  String get id {
    _$idAtom.reportRead();
    return super.id;
  }

  @override
  set id(String value) {
    _$idAtom.reportWrite(value, super.id, () {
      super.id = value;
    });
  }

  final _$titleAtom = Atom(name: '_Clip.title');

  @override
  String get title {
    _$titleAtom.reportRead();
    return super.title;
  }

  @override
  set title(String value) {
    _$titleAtom.reportWrite(value, super.title, () {
      super.title = value;
    });
  }

  @override
  String toString() {
    return '''
id: ${id},
title: ${title}
    ''';
  }
}
