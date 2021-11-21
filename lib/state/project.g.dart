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
  String toString() {
    return '''
tracks: ${tracks}
    ''';
  }
}

mixin _$Track on _Track, Store {
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

  @override
  String toString() {
    return '''
id: ${id},
title: ${title}
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
