// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'history_state_model.dart';

// **************************************************************************
// StoreGenerator
// **************************************************************************

// ignore_for_file: non_constant_identifier_names, unnecessary_brace_in_string_interps, unnecessary_lambdas, prefer_expression_function_bodies, lines_longer_than_80_chars, avoid_as, avoid_annotating_with_dynamic, no_leading_underscores_for_local_identifiers

mixin _$HistoryStateModel on _HistoryStateModel, Store {
  Computed<HistoryChartModel>? _$chartModelComputed;

  @override
  HistoryChartModel get chartModel => (_$chartModelComputed ??=
          Computed<HistoryChartModel>(() => super.chartModel,
              name: '_HistoryStateModel.chartModel'))
      .value;

  late final _$sessionsAtom =
      Atom(name: '_HistoryStateModel.sessions', context: context);

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

  late final _$dailyPracticeTimeAtom =
      Atom(name: '_HistoryStateModel.dailyPracticeTime', context: context);

  @override
  ObservableList<DailyPracticeTime> get dailyPracticeTime {
    _$dailyPracticeTimeAtom.reportRead();
    return super.dailyPracticeTime;
  }

  @override
  set dailyPracticeTime(ObservableList<DailyPracticeTime> value) {
    _$dailyPracticeTimeAtom.reportWrite(value, super.dailyPracticeTime, () {
      super.dailyPracticeTime = value;
    });
  }

  late final _$weeklyPracticeTimeAtom =
      Atom(name: '_HistoryStateModel.weeklyPracticeTime', context: context);

  @override
  ObservableList<DailyPracticeTime> get weeklyPracticeTime {
    _$weeklyPracticeTimeAtom.reportRead();
    return super.weeklyPracticeTime;
  }

  @override
  set weeklyPracticeTime(ObservableList<DailyPracticeTime> value) {
    _$weeklyPracticeTimeAtom.reportWrite(value, super.weeklyPracticeTime, () {
      super.weeklyPracticeTime = value;
    });
  }

  late final _$historyResolutionAtom =
      Atom(name: '_HistoryStateModel.historyResolution', context: context);

  @override
  HistoryResolution get historyResolution {
    _$historyResolutionAtom.reportRead();
    return super.historyResolution;
  }

  @override
  set historyResolution(HistoryResolution value) {
    _$historyResolutionAtom.reportWrite(value, super.historyResolution, () {
      super.historyResolution = value;
    });
  }

  @override
  String toString() {
    return '''
sessions: ${sessions},
dailyPracticeTime: ${dailyPracticeTime},
weeklyPracticeTime: ${weeklyPracticeTime},
historyResolution: ${historyResolution},
chartModel: ${chartModel}
    ''';
  }
}
