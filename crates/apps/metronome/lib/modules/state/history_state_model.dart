import 'package:mobx/mobx.dart';

import '../history/session_entity.dart';

part 'history_state_model.g.dart';

class HistoryStateModel = _HistoryStateModel with _$HistoryStateModel;

abstract class _HistoryStateModel with Store {
  @observable
  ObservableList<Session> sessions = ObservableList.of([]);
}
