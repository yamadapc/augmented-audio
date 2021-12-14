abstract class AudioIOStore {
  Future<String> getInputDevices();
}

abstract class AudioThread {
  Future<void> setOptions(
      {required String inputDeviceId, required String outputDeviceId});
}

abstract class AudioGraph {
  Future<List<int>> systemIndexes();
  Future<int> createNode({required String name});
  Future<int> connect({required int inputIndex, required int outputIndex});
}
