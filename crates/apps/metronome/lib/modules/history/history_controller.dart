import 'package:clock/clock.dart';
import 'package:metronome/logger.dart';
import 'package:metronome/modules/history/session_dao.dart';
import 'package:metronome/modules/history/session_entity.dart';
import 'package:metronome/modules/state/history_state_controller.dart';
import 'package:metronome/modules/state/metronome_state_model.dart';

const thresholdMs = 1000;

class HistoryStartStopHandler {
  final SessionDao sessionDao;
  final MetronomeStateModel model;
  final HistoryStateController historyStateController;

  DateTime? start;

  HistoryStartStopHandler(
    this.sessionDao,
    this.model,
    this.historyStateController,
  );

  void onStart() {
    start = clock.now();
    logger.i("Session start=$start");
  }

  Future<void> onEnd() async {
    if (start == null) {
      return;
    }

    final now = clock.now();
    final duration = now.difference(start!);
    final durationMs = duration.inMilliseconds;

    if (durationMs < thresholdMs) {
      return;
    }

    logger.i("Session end durationMs=$durationMs");
    final session = Session.create(
      timestampMs: start!.millisecondsSinceEpoch,
      durationMs: durationMs,
      tempo: model.tempo,
      beatsPerBar: model.beatsPerBar,
    );
    await sessionDao.insertSession(session);
    await historyStateController.refresh();

    start = null;
  }
}
