// import 'package:firebase_performance/firebase_performance.dart';
import 'package:metronome/logger.dart';
import 'package:metronome/modules/history/session_dao.dart';
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
    logger.i("Refreshing sessions from DB length=${sessions.length}");

    runInAction(() {
      _historyStateModel.sessions.clear();
      _historyStateModel.sessions.addAll(sessions);

      // trace.setMetric("numSessions", sessions.length);
      // trace.stop();
    });
  }
}
