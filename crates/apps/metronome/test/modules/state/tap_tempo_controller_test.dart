import 'package:flutter_test/flutter_test.dart';
import 'package:metronome/modules/state/tap_tempo_controller.dart';

import '../../mock_metronome.dart';

/// This is a useless test at the moment
void main() {
  test("TapTempoController create", () async {
    var env = await buildTestEnvironment();
    var controller = TapTempoController(env.metronomeStateController);
    controller.onPress();
  });
}
