import 'package:flutter_daw_mock_ui/services/state_sync.dart';
import 'package:mobx/mobx.dart';

abstract class Entity {
  String get id;

  ActionController getActionController() {
    return TrackedActionController(id);
  }
}
