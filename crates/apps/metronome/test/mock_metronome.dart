import 'package:flutter_rust_bridge/src/platform_independent.dart';
import 'package:metronome/bridge_generated.dart';
import 'package:metronome/logger.dart';
import 'package:metronome/modules/analytics/fake_analytics.dart';
import 'package:metronome/modules/database.dart';
import 'package:metronome/modules/history/history_controller.dart';
import 'package:metronome/modules/state/history_state_controller.dart';
import 'package:metronome/modules/state/history_state_model.dart';
import 'package:metronome/modules/state/metronome_state_controller.dart';
import 'package:metronome/modules/state/metronome_state_model.dart';

class MockMetronomeLib implements Metronome {
  @override
  Future<int> deinitialize({dynamic hint}) {
    return Future.value(0);
  }

  @override
  Stream<double> getPlayhead({dynamic hint}) {
    return const Stream.empty();
  }

  @override
  Future<int> initialize({required InitializeOptions options, dynamic hint}) {
    return Future.value(0);
  }

  @override
  Future<int> setBeatsPerBar({required int value, dynamic hint}) {
    return Future.value(0);
  }

  @override
  Future<int> setIsPlaying({required bool value, dynamic hint}) {
    return Future.value(0);
  }

  @override
  Future<int> setTempo({required double value, dynamic hint}) {
    return Future.value(0);
  }

  @override
  Future<int> setVolume({required double value, dynamic hint}) {
    return Future.value(0);
  }

  @override
  FlutterRustBridgeTaskConstMeta get kDeinitializeConstMeta =>
      throw UnimplementedError();

  @override
  FlutterRustBridgeTaskConstMeta get kGetPlayheadConstMeta =>
      throw UnimplementedError();

  @override
  FlutterRustBridgeTaskConstMeta get kInitializeConstMeta =>
      throw UnimplementedError();

  @override
  FlutterRustBridgeTaskConstMeta get kSetBeatsPerBarConstMeta =>
      throw UnimplementedError();

  @override
  FlutterRustBridgeTaskConstMeta get kSetIsPlayingConstMeta =>
      throw UnimplementedError();

  @override
  FlutterRustBridgeTaskConstMeta get kSetTempoConstMeta =>
      throw UnimplementedError();

  @override
  FlutterRustBridgeTaskConstMeta get kSetVolumeConstMeta =>
      throw UnimplementedError();

  @override
  FlutterRustBridgeTaskConstMeta get kSetSoundConstMeta =>
      throw UnimplementedError();

  @override
  Future<int> setSound({required MetronomeSoundTypeTag value, dynamic hint}) {
    return Future.value(0);
  }
}

Future<MockEnvironment> buildTestEnvironment() async {
  // Set-up mocked environment
  logger.i("Setting-up mock environment");
  final model = MetronomeStateModel();
  final metronome = MockMetronomeLib();
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
    metronome,
    historyStartStopHandler,
    FakeAnalytics(),
  );

  return MockEnvironment.create(
    model,
    metronome,
    database,
    historyStateModel,
    historyStateController,
    historyStartStopHandler,
    metronomeStateController,
  );
}

class MockEnvironment {
  MetronomeStateModel model;
  Metronome metronome;
  MetronomeDatabase database;
  HistoryStateModel historyStateModel;
  HistoryStateController historyStateController;
  HistoryStartStopHandler historyStartStopHandler;
  MetronomeStateController metronomeStateController;

  MockEnvironment.create(
    this.model,
    this.metronome,
    this.database,
    this.historyStateModel,
    this.historyStateController,
    this.historyStartStopHandler,
    this.metronomeStateController,
  );
}
