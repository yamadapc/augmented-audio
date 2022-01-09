import 'package:metronome/modules/history/session_dao.dart';
import 'package:metronome/modules/history/session_entity.dart';
import 'package:metronome/modules/state/history_state_controller.dart';
import 'package:metronome/modules/state/metronome_state_model.dart';

import '../../logger.dart';

const thresholdMs = 1000;

class HistoryStartStopHandler {
  final SessionDao sessionDao;
  final MetronomeStateModel model;
  final HistoryStateController historyStateController;

  DateTime? start;

  HistoryStartStopHandler(this.sessionDao, this.model,
      this.historyStateController);

  void onStart() {
    start = DateTime.now();
    logger.i("Session start=$start");
  }

  void onEnd() async {
    if (start == null) {
      return;
    }

    var now = DateTime.now();
    var duration = now.difference(start!);
    var durationMs = duration.inMilliseconds;

    if (durationMs < thresholdMs) {
      return;
    }

    logger.i("Session end durationMs=$durationMs");
    var session =
    Session(null, start!.millisecondsSinceEpoch, durationMs, model.tempo);
    await sessionDao.insertSession(session);
    await historyStateController.refresh();

    start = null;
  }
}
