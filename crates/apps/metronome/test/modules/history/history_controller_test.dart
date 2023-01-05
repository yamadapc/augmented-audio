import 'package:clock/clock.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:metronome/modules/history/history_controller.dart';
import 'package:metronome/modules/history/session_dao.mocks.dart';
import 'package:metronome/modules/state/history_state_controller.mocks.dart';
import 'package:metronome/modules/state/metronome_state_model.dart';
import 'package:mockito/mockito.dart';

void main() {
  test("HistoryStartStopHandler::onStart updates the current start time", () {
    final sessionDao = MockSessionDao();
    final model = MetronomeStateModel();
    final historyStateController = MockHistoryStateController();

    final HistoryStartStopHandler handler = HistoryStartStopHandler(
      sessionDao,
      model,
      historyStateController,
    );

    withClock(Clock.fixed(DateTime(2000)), () {
      handler.onStart();
      expect(handler.start, equals(DateTime(2000)));
    });
  });

  test("HistoryStartStopHandler::onEnd stores no entry if start is null", () {
    final sessionDao = MockSessionDao();
    final model = MetronomeStateModel();
    final historyStateController = MockHistoryStateController();

    final HistoryStartStopHandler handler = HistoryStartStopHandler(
      sessionDao,
      model,
      historyStateController,
    );

    when(sessionDao.insertSession(any))
        .thenThrow(Exception("Should not be called"));
    when(historyStateController.refresh())
        .thenThrow(Exception("Should not be called"));

    expect(handler.start, isNull);
    handler.onEnd();
    expect(handler.start, isNull);
  });

  test(
      "HistoryStartStopHandler::onEnd stores no entry if start under the threshold",
      () async {
    final sessionDao = MockSessionDao();
    final model = MetronomeStateModel();
    final historyStateController = MockHistoryStateController();

    final HistoryStartStopHandler handler = HistoryStartStopHandler(
      sessionDao,
      model,
      historyStateController,
    );

    withClock(Clock.fixed(DateTime(2000)), () {
      handler.onStart();
    });

    when(sessionDao.insertSession(any)).thenAnswer((inv) => Future.value(1));
    when(historyStateController.refresh()).thenAnswer((inv) => Future.value());

    await withClock(
      Clock.fixed(DateTime(2000).add(const Duration(milliseconds: 500))),
      () => handler.onEnd(),
    );

    expect(handler.start, equals(DateTime(2000)));
    verifyNever(sessionDao.insertSession(any));
    verifyNever(historyStateController.refresh());
  });

  test(
      "HistoryStartStopHandler::onEnd stores an entry if start is not null and above threshold",
      () async {
    final sessionDao = MockSessionDao();
    final model = MetronomeStateModel();
    final historyStateController = MockHistoryStateController();

    final HistoryStartStopHandler handler = HistoryStartStopHandler(
      sessionDao,
      model,
      historyStateController,
    );

    withClock(Clock.fixed(DateTime(2000)), () {
      handler.onStart();
    });

    when(sessionDao.insertSession(any)).thenAnswer((inv) => Future.value(1));
    when(historyStateController.refresh()).thenAnswer((inv) => Future.value());

    await withClock(
      Clock.fixed(DateTime(2000).add(const Duration(seconds: 3))),
      () => handler.onEnd(),
    );

    expect(handler.start, isNull);
    verify(sessionDao.insertSession(any)).called(1);
    verify(historyStateController.refresh()).called(1);
  });
}
