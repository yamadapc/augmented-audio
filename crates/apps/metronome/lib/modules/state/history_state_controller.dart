// import 'package:firebase_performance/firebase_performance.dart';
import 'package:metronome/logger.dart';
import 'package:metronome/modules/history/session_dao.dart';
import 'package:metronome/modules/history/session_entity.dart';
import 'package:metronome/modules/state/history_state_model.dart';
import 'package:mobx/mobx.dart';

class HistoryStateController {
  final SessionDao _sessionDao;
  final HistoryStateModel _historyStateModel;

  HistoryStateController(this._sessionDao, this._historyStateModel);

  HistoryStateModel get model {
    return _historyStateModel;
  }

  Future<void> refresh() async {
    // var performance = FirebasePerformance.instance;
    // var trace = performance.newTrace("HistoryStateController::refresh");
    // trace.start();

    final lastWeek =
        DateTime.now().millisecondsSinceEpoch - 1000 * 60 * 60 * 24 * 7;
    final sessions = await _sessionDao.findAggregatedSessions(lastWeek);
    final lastTwoMonths =
        DateTime.now().millisecondsSinceEpoch - 1000 * 60 * 60 * 24 * 60;
    final dailyTime = await _sessionDao.findDailyPracticeTime(lastTwoMonths);
    final Map<int, int> timePerWeek = {};
    for (final practiceTime in dailyTime) {
      final timestampMs = startOfWeek(
        DateTime.fromMillisecondsSinceEpoch(practiceTime.timestampMs),
      ).millisecondsSinceEpoch;
      timePerWeek.update(
        timestampMs,
        (value) => value += practiceTime.durationMs,
        ifAbsent: () => 0,
      );
    }
    final weeklyTime = timePerWeek.entries
        .map((e) => DailyPracticeTime(e.value, e.key))
        .toList();
    weeklyTime.sort(
      (entry1, entry2) => entry1.timestampMs > entry2.timestampMs ? 1 : -1,
    );

    logger.i("Refreshing sessions from DB length=${sessions.length}");
    logger.i("$weeklyTime");
    runInAction(() {
      _historyStateModel.sessions.clear();
      _historyStateModel.sessions.addAll(sessions);
      _historyStateModel.dailyPracticeTime.clear();
      _historyStateModel.dailyPracticeTime.addAll(dailyTime);
      _historyStateModel.weeklyPracticeTime.clear();
      _historyStateModel.weeklyPracticeTime.addAll(weeklyTime);

      // trace.setMetric("numSessions", sessions.length);
      // trace.stop();
    });
  }
}

DateTime startOfWeek(DateTime dateTime) {
  return dateTime.subtract(Duration(days: dateTime.weekday - 1));
}
