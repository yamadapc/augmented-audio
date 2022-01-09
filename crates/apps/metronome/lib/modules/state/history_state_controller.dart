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
    final sessions = await _sessionDao.findAllSessions();
    logger.i("Refreshing sessions from DB length=${sessions.length}");

    runInAction(() {
      _historyStateModel.sessions.clear();
      _historyStateModel.sessions.addAll(sessions);
    });
  }
}
