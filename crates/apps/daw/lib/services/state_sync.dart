import 'package:flutter_daw_mock_ui/bridge_generated.dart';
import 'package:flutter_daw_mock_ui/state/entity.dart';
import 'package:flutter_daw_mock_ui/state/wire.dart';
import 'package:mobx/mobx.dart';

StateSyncService? instance;

List<Dispose> disposeCallbacks = [];

class TrackedActionController extends ActionController {
  final String entityId;
  final StateSyncService stateSyncService = StateSyncService.get(dawUi!);

  TrackedActionController(this.entityId) : super();

  @override
  ActionRunInfo startAction({String? name}) {
    stateSyncService.entityActionStack.add(entityId);
    return super.startAction(name: name);
  }

  @override
  void endAction(ActionRunInfo info) {
    stateSyncService.entityActionStack.removeLast();
    super.endAction(info);
  }
}

class StateSyncService {
  final DawUi api;
  final List<String> entityActionStack = [];

  StateSyncService(this.api);

  void start() {
    for (var dispose in disposeCallbacks) {
      dispose();
    }

    mainContext.config = ReactiveConfig.main.clone(isSpyEnabled: true);
    var dispose = mainContext.spy((event) {
      onEvent(event);
    });
    disposeCallbacks.add(dispose);

    // var stream = api.getEventsSink();
    // stream.forEach((element) {
    //   print("REMOTE EVENT - $element");
    // });
  }

  void onEvent(SpyEvent event) {
    if (event is ObservableValueSpyEvent) {
      var entityId = entityActionStack.isEmpty ? null : entityActionStack.last;
      if (entityId != null) {
        var changePath = entityId + "/" + event.name.split(".")[1];
        var newValue =
            event.newValue is Entity ? event.newValue.id : event.newValue;
        logChange(changePath, newValue);
      }
    }
  }

  void logChange(changePath, newValue) {
    // print("$changePath = $newValue");
  }

  static get(DawUi api) {
    if (instance != null) {
      return instance;
    }

    instance = StateSyncService(api);
    return instance;
  }
}

T runInEntity<T>(Entity entity, T Function() fn) {
  var stateSyncService = StateSyncService.get(dawUi!);
  stateSyncService.entityActionStack.add(entity.id);
  var result = runInAction(fn);
  stateSyncService.entityActionStack.removeLast();
  return result;
}
