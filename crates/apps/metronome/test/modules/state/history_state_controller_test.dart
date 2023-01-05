import 'package:flutter_test/flutter_test.dart';
import 'package:metronome/modules/history/session_dao.mocks.dart';
import 'package:metronome/modules/state/history_state_controller.dart';
import 'package:metronome/modules/state/history_state_model.dart';

void main() {
  test("startOfWeek returns the start of that week", () {
    final start = startOfWeek(DateTime(2021));
    expect(start, DateTime(2020, 12, 28));
  });

  test("HistoryStateController refresh", () async {
    final sessionDao = MockSessionDao();
    final historyStateModel = HistoryStateModel();
    final historyStateController =
        HistoryStateController(sessionDao, historyStateModel);

    await historyStateController.refresh();
  });
}
