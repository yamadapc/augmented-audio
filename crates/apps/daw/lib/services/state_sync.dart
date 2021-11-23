import 'package:flutter_daw_mock_ui/state/entity.dart';
import 'package:mobx/mobx.dart';

StateSyncService? instance;

List<Dispose> disposeCallbacks = [];

class TrackedActionController extends ActionController {
  final String entityId;
  final StateSyncService stateSyncService = StateSyncService.get();

  TrackedActionController(this.entityId) : super();

  @override
  ActionRunInfo startAction({String? name}) {
    stateSyncService.pushEntity(entityId);
    return super.startAction(name: name);
  }

  @override
  void endAction(ActionRunInfo info) {
    stateSyncService.popEntity();
    super.endAction(info);
  }
}

class StateSyncService {
  final List<String> _entityActionStack = [];

  StateSyncService();

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
      var entityId =
          _entityActionStack.isEmpty ? null : _entityActionStack.last;
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

  void pushEntity(String entityId) {
    _entityActionStack.add(entityId);
  }

  void popEntity() {
    _entityActionStack.removeLast();
  }

  static get() {
    if (instance != null) {
      return instance;
    }

    instance = StateSyncService();
    return instance;
  }
}

T runInEntity<T>(Entity entity, T Function() fn) {
  StateSyncService stateSyncService = StateSyncService.get();
  stateSyncService.pushEntity(entity.id);
  var result = runInAction(fn);
  stateSyncService.popEntity();
  return result;
}
