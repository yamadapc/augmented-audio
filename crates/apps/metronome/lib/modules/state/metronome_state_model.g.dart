// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'metronome_state_model.dart';

// **************************************************************************
// StoreGenerator
// **************************************************************************

// ignore_for_file: non_constant_identifier_names, unnecessary_brace_in_string_interps, unnecessary_lambdas, prefer_expression_function_bodies, lines_longer_than_80_chars, avoid_as, avoid_annotating_with_dynamic

mixin _$MetronomeStateModel on _MetronomeStateModel, Store {
  final _$isPlayingAtom = Atom(name: '_MetronomeStateModel.isPlaying');

  @override
  bool get isPlaying {
    _$isPlayingAtom.reportRead();
    return super.isPlaying;
  }

  @override
  set isPlaying(bool value) {
    _$isPlayingAtom.reportWrite(value, super.isPlaying, () {
      super.isPlaying = value;
    });
  }

  final _$volumeAtom = Atom(name: '_MetronomeStateModel.volume');

  @override
  double get volume {
    _$volumeAtom.reportRead();
    return super.volume;
  }

  @override
  set volume(double value) {
    _$volumeAtom.reportWrite(value, super.volume, () {
      super.volume = value;
    });
  }

  final _$tempoAtom = Atom(name: '_MetronomeStateModel.tempo');

  @override
  double get tempo {
    _$tempoAtom.reportRead();
    return super.tempo;
  }

  @override
  set tempo(double value) {
    _$tempoAtom.reportWrite(value, super.tempo, () {
      super.tempo = value;
    });
  }

  final _$playheadAtom = Atom(name: '_MetronomeStateModel.playhead');

  @override
  double get playhead {
    _$playheadAtom.reportRead();
    return super.playhead;
  }

  @override
  set playhead(double value) {
    _$playheadAtom.reportWrite(value, super.playhead, () {
      super.playhead = value;
    });
  }

  final _$beatsPerBarAtom = Atom(name: '_MetronomeStateModel.beatsPerBar');

  @override
  int get beatsPerBar {
    _$beatsPerBarAtom.reportRead();
    return super.beatsPerBar;
  }

  @override
  set beatsPerBar(int value) {
    _$beatsPerBarAtom.reportWrite(value, super.beatsPerBar, () {
      super.beatsPerBar = value;
    });
  }

  final _$_MetronomeStateModelActionController =
      ActionController(name: '_MetronomeStateModel');

  @override
  void setPlayhead(double value) {
    final _$actionInfo = _$_MetronomeStateModelActionController.startAction(
        name: '_MetronomeStateModel.setPlayhead');
    try {
      return super.setPlayhead(value);
    } finally {
      _$_MetronomeStateModelActionController.endAction(_$actionInfo);
    }
  }

  @override
  void setTempo(double value) {
    final _$actionInfo = _$_MetronomeStateModelActionController.startAction(
        name: '_MetronomeStateModel.setTempo');
    try {
      return super.setTempo(value);
    } finally {
      _$_MetronomeStateModelActionController.endAction(_$actionInfo);
    }
  }

  @override
  void setIsPlaying(bool value) {
    final _$actionInfo = _$_MetronomeStateModelActionController.startAction(
        name: '_MetronomeStateModel.setIsPlaying');
    try {
      return super.setIsPlaying(value);
    } finally {
      _$_MetronomeStateModelActionController.endAction(_$actionInfo);
    }
  }

  @override
  void setVolume(double value) {
    final _$actionInfo = _$_MetronomeStateModelActionController.startAction(
        name: '_MetronomeStateModel.setVolume');
    try {
      return super.setVolume(value);
    } finally {
      _$_MetronomeStateModelActionController.endAction(_$actionInfo);
    }
  }

  @override
  void setBeatsPerBar(int value) {
    final _$actionInfo = _$_MetronomeStateModelActionController.startAction(
        name: '_MetronomeStateModel.setBeatsPerBar');
    try {
      return super.setBeatsPerBar(value);
    } finally {
      _$_MetronomeStateModelActionController.endAction(_$actionInfo);
    }
  }

  @override
  String toString() {
    return '''
isPlaying: ${isPlaying},
volume: ${volume},
tempo: ${tempo},
playhead: ${playhead},
beatsPerBar: ${beatsPerBar}
    ''';
  }
}
