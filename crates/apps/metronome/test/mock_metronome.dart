import 'package:metronome/logger.dart';
import 'package:metronome/modules/analytics/fake_analytics.dart';
import 'package:metronome/modules/database.dart';
import 'package:metronome/modules/history/history_controller.dart';
import 'package:metronome/modules/state/history_state_controller.dart';
import 'package:metronome/modules/state/history_state_model.dart';
import 'package:metronome/modules/state/metronome_state_controller.dart';
import 'package:metronome/modules/state/metronome_state_model.dart';
import 'package:metronome/src/rust/api.dart';
import 'package:metronome/src/rust/frb_generated.dart';
import 'package:metronome/src/rust/internal/processor.dart';
import 'package:metronome/src/rust/internal/state.dart';

Future<MockEnvironment> buildTestEnvironment() async {
  // Set-up mocked environment
  logger.i("Setting-up mock environment");
  final model = MetronomeStateModel();
  final mockApi = MockRustLibApi();
  RustLib.initMock(api: mockApi);
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

class MockRustLibApi implements RustLibApi {
  @override
  Future<int> crateApiDeinitialize() {
    return Future.value(0);
  }

  @override
  Future<double> crateApiGetPlayhead() {
    return Future.value(0);
  }

  @override
  Future<int> crateApiInitialize({required InitializeOptions options}) {
    return Future.value(0);
  }

  @override
  Future<int> crateApiSetBeatsPerBar({required int value}) {
    return Future.value(0);
  }

  @override
  Future<int> crateApiSetIsPlaying({required bool value}) {
    return Future.value(0);
  }

  @override
  Future<int> crateApiSetSound({required MetronomeSoundTypeTag value}) {
    return Future.value(0);
  }

  @override
  Future<int> crateApiSetTempo({required double value}) {
    return Future.value(0);
  }

  @override
  Future<int> crateApiSetVolume({required double value}) {
    return Future.value(0);
  }

  @override
  Stream<EngineError> crateApiStreamErrors() {
    return Stream.empty();
  }
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
