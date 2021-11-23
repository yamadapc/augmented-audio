import 'wire_base.dart';
import 'wire_native.dart' if (dart.library.html) 'wire_web.dart' as WireImpl;

const initialize = WireImpl.initialize;
var dawUi = WireImpl.dawUi;

AudioIOStore? getAudioIOStore() {
  return WireImpl.getAudioIOStore();
}
