import 'package:flutter/foundation.dart';
import 'package:metronome/logger.dart';
import 'package:metronome/modules/history/session_dao.dart';
import 'package:metronome/modules/history/session_entity.dart';
import 'package:metronome/modules/performance_metrics/metrics.dart';
import 'package:metronome/modules/state/history_state_model.dart';
import 'package:mobx/mobx.dart';
import 'package:mockito/annotations.dart';

@GenerateNiceMocks(
  [MockSpec<HistoryStateController>()],
)
// ignore: unused_import, always_use_package_imports
import './history_state_controller.mocks.dart';

class HistoryStateController {
  final SessionDao _sessionDao;
  final HistoryStateModel _historyStateModel;

  HistoryStateController(this._sessionDao, this._historyStateModel);

  HistoryStateModel get model {
    return _historyStateModel;
  }

  /// Fetch recent practice sessions from the DB and push them into in-memory
  /// state.
  Future<void> refresh() async {
    final performance = getMetrics();
    final trace = performance.newTrace("HistoryStateController::refresh");
    trace.start();

    final List<AggregatedSession> sessions = await findRecentSessions();
    final List<DailyPracticeTime> dailyTime =
        await findRecentDailyPracticeTime();
    final List<DailyPracticeTime> weeklyTime =
        aggregateWeeklyPracticeTime(dailyTime);

    logger.i("Refreshing sessions from DB length=${sessions.length}");
    logger.i("$weeklyTime");
    runInAction(() {
      _historyStateModel.sessions.clear();
      _historyStateModel.sessions.addAll(sessions);
      _historyStateModel.dailyPracticeTime.clear();
      _historyStateModel.dailyPracticeTime.addAll(dailyTime);
      _historyStateModel.weeklyPracticeTime.clear();
      _historyStateModel.weeklyPracticeTime.addAll(weeklyTime);

      trace.setMetric("numSessions", sessions.length);
      trace.stop();
    });
  }

  /// Find the last two months of daily practice time
  @visibleForTesting
  Future<List<DailyPracticeTime>> findRecentDailyPracticeTime() async {
    final lastTwoMonths =
        DateTime.now().millisecondsSinceEpoch - 1000 * 60 * 60 * 24 * 60;
    final dailyTime = await _sessionDao.findDailyPracticeTime(lastTwoMonths);
    return dailyTime;
  }

  /// Find the 100 practice entries
  @visibleForTesting
  Future<List<AggregatedSession>> findRecentSessions() async {
    final sessions = await _sessionDao.findAggregatedSessions();
    return sessions;
  }
}

/// Group daily practice time by week and create a list of the weekly practice
/// time with the sum.
@visibleForTesting
List<DailyPracticeTime> aggregateWeeklyPracticeTime(
  List<DailyPracticeTime> dailyTime,
) {
  final Map<int, int> timePerWeek = {};
  for (final practiceTime in dailyTime) {
    final timestampMs = ChartTransformer.startOfDate(
      date: DateTime.fromMillisecondsSinceEpoch(practiceTime.timestampMs),
      resolution: HistoryResolution.weeks,
    ).millisecondsSinceEpoch;

    timePerWeek.update(
      timestampMs,
      (value) => value + practiceTime.durationMs,
      ifAbsent: () => practiceTime.durationMs,
    );
  }
  final weeklyTime = timePerWeek.entries
      .map((e) => DailyPracticeTime(e.value, e.key))
      .toList();
  weeklyTime.sort(
    (entry1, entry2) => entry1.timestampMs > entry2.timestampMs ? 1 : -1,
  );
  return weeklyTime;
}
