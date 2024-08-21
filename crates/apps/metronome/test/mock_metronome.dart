import 'package:metronome/logger.dart';
import 'package:metronome/modules/analytics/fake_analytics.dart';
import 'package:metronome/modules/database.dart';
import 'package:metronome/modules/history/history_controller.dart';
import 'package:metronome/modules/state/history_state_controller.dart';
import 'package:metronome/modules/state/history_state_model.dart';
import 'package:metronome/modules/state/metronome_state_controller.dart';
import 'package:metronome/modules/state/metronome_state_model.dart';
import 'package:metronome/src/rust/frb_generated.dart';

Future<MockEnvironment> buildTestEnvironment() async {
  // Set-up mocked environment
  logger.i("Setting-up mock environment");
  final model = MetronomeStateModel();
  await RustLib.init();
  final database = await buildInMemoryDatabase();
  final sessionDao = database.sessionDao;
  final historyStateModel = HistoryStateModel();
  final historyStateController = HistoryStateController(
    sessionDao,
    historyStateModel,
  );
  final historyStartStopHandler = HistoryStartStopHandler(
    sessionDao,
    model,
    historyStateController,
  );
  final metronomeStateController = MetronomeStateController(
    model,
    historyStartStopHandler,
    FakeAnalytics(),
  );

  return MockEnvironment.create(
    model,
    database,
    historyStateModel,
    historyStateController,
    historyStartStopHandler,
    metronomeStateController,
  );
}

class MockEnvironment {
  MetronomeStateModel model;
  MetronomeDatabase database;
  HistoryStateModel historyStateModel;
  HistoryStateController historyStateController;
  HistoryStartStopHandler historyStartStopHandler;
  MetronomeStateController metronomeStateController;

  MockEnvironment.create(
    this.model,
    this.database,
    this.historyStateModel,
    this.historyStateController,
    this.historyStartStopHandler,
    this.metronomeStateController,
  );
}
