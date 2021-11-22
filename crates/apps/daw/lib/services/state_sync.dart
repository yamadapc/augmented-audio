import 'package:flutter_daw_mock_ui/bridge_generated.dart';
import 'package:mobx/mobx.dart';

StateSyncService? instance;

List<Dispose> disposeCallbacks = [];

class StateSyncService {
  final DawUi api;

  StateSyncService(this.api);

  void start() {
    for (var dispose in disposeCallbacks) {
      dispose();
    }

    mainContext.config = ReactiveConfig.main.clone(isSpyEnabled: false);
    // var dispose = mainContext.spy((event) {
    //   onEvent(event);
    // });
    // disposeCallbacks.add(dispose);
  }

  // void onEvent(SpyEvent event) {
  //   if (event is ObservableValueSpyEvent) {
  //     print(
  //         "${event.object} - ${event.name} - ${event.oldValue} - ${event.newValue}");
  //   }
  // }

  static get(DawUi api) {
    if (instance != null) {
      return instance;
    }

    instance = StateSyncService(api);
    return instance;
  }
}
