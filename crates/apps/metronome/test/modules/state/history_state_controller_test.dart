import 'package:flutter_test/flutter_test.dart';
import 'package:metronome/modules/history/session_dao.mocks.dart';
import 'package:metronome/modules/state/history_state_controller.dart';
import 'package:metronome/modules/state/history_state_model.dart';

void main() {
  test("HistoryStateController refresh", () async {
    final sessionDao = MockSessionDao();
    final historyStateModel = HistoryStateModel();
    final historyStateController =
        HistoryStateController(sessionDao, historyStateModel);

    await historyStateController.refresh();
  });
}
