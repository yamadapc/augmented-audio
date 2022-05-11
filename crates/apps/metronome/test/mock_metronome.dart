import 'package:metronome/bridge_generated.dart';

class MockMetronomeLib implements Metronome {
  @override
  Future<int> deinitialize({hint}) {
    return Future.value(0);
  }

  @override
  Stream<double> getPlayhead({hint}) {
    return const Stream.empty();
  }

  @override
  Future<int> initialize({hint}) {
    return Future.value(0);
  }

  @override
  Future<int> setBeatsPerBar({required int value, hint}) {
    return Future.value(0);
  }

  @override
  Future<int> setIsPlaying({required bool value, hint}) {
    return Future.value(0);
  }

  @override
  Future<int> setTempo({required double value, hint}) {
    return Future.value(0);
  }

  @override
  Future<int> setVolume({required double value, hint}) {
    return Future.value(0);
  }
}
