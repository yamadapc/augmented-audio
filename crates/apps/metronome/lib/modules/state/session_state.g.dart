// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'session_state.dart';

// **************************************************************************
// StoreGenerator
// **************************************************************************

// ignore_for_file: non_constant_identifier_names, unnecessary_brace_in_string_interps, unnecessary_lambdas, prefer_expression_function_bodies, lines_longer_than_80_chars, avoid_as, avoid_annotating_with_dynamic, no_leading_underscores_for_local_identifiers

mixin _$SessionState on _SessionState, Store {
  late final _$isPlayingAtom =
      Atom(name: '_SessionState.isPlaying', context: context);

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

  late final _$startAtom = Atom(name: '_SessionState.start', context: context);

  @override
  DateTime? get start {
    _$startAtom.reportRead();
    return super.start;
  }

  @override
  set start(DateTime? value) {
    _$startAtom.reportWrite(value, super.start, () {
      super.start = value;
    });
  }

  late final _$nowAtom = Atom(name: '_SessionState.now', context: context);

  @override
  DateTime get now {
    _$nowAtom.reportRead();
    return super.now;
  }

  @override
  set now(DateTime value) {
    _$nowAtom.reportWrite(value, super.now, () {
      super.now = value;
    });
  }

  late final _$timerAtom = Atom(name: '_SessionState.timer', context: context);

  @override
  Timer? get timer {
    _$timerAtom.reportRead();
    return super.timer;
  }

  @override
  set timer(Timer? value) {
    _$timerAtom.reportWrite(value, super.timer, () {
      super.timer = value;
    });
  }

  late final _$_SessionStateActionController =
      ActionController(name: '_SessionState', context: context);

  @override
  void startSession() {
    final _$actionInfo = _$_SessionStateActionController.startAction(
        name: '_SessionState.startSession');
    try {
      return super.startSession();
    } finally {
      _$_SessionStateActionController.endAction(_$actionInfo);
    }
  }

  @override
  void stopSession() {
    final _$actionInfo = _$_SessionStateActionController.startAction(
        name: '_SessionState.stopSession');
    try {
      return super.stopSession();
    } finally {
      _$_SessionStateActionController.endAction(_$actionInfo);
    }
  }

  @override
  String toString() {
    return '''
isPlaying: ${isPlaying},
start: ${start},
now: ${now},
timer: ${timer}
    ''';
  }
}
