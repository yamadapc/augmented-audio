// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'history_state_model.dart';

// **************************************************************************
// StoreGenerator
// **************************************************************************

// ignore_for_file: non_constant_identifier_names, unnecessary_brace_in_string_interps, unnecessary_lambdas, prefer_expression_function_bodies, lines_longer_than_80_chars, avoid_as, avoid_annotating_with_dynamic

mixin _$HistoryStateModel on _HistoryStateModel, Store {
  final _$sessionsAtom = Atom(name: '_HistoryStateModel.sessions');

  @override
  ObservableList<AggregatedSession> get sessions {
    _$sessionsAtom.reportRead();
    return super.sessions;
  }

  @override
  set sessions(ObservableList<AggregatedSession> value) {
    _$sessionsAtom.reportWrite(value, super.sessions, () {
      super.sessions = value;
    });
  }

  @override
  String toString() {
    return '''
sessions: ${sessions}
    ''';
  }
}
