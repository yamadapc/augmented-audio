import 'package:flutter_daw_mock_ui/ui/main_content_layout/midi_editor/midi_model.dart';
import 'package:graphx/graphx.dart';
import 'package:mobx/mobx.dart';

import 'entity.dart';

part 'ui_state.g.dart';

class UIState = _UIState with _$UIState;

abstract class _UIState with Store {
  @observable
  SidebarState sidebarState = SidebarState(id: "/UI/Sidebar");

  @observable
  PanelTabsState mainContentTabsState =
      PanelTabsState("/UI/mainContentTabsState/PanelTabsState");

  @observable
  DetailsPanelState detailsPanelState = DetailsPanelState();
}

class SidebarState = _SidebarState with _$SidebarState;

abstract class _SidebarState with Store, Entity {
  @override
  String id;

  @observable
  PanelState panelState = PanelState("PanelState");

  _SidebarState({required this.id, PanelState? panelState}) {
    this.panelState = panelState ?? PanelState(id + "/PanelState");
  }
}

class PanelState extends _PanelState with _$PanelState {
  PanelState(String id) {
    this.id = id;
  }

  @override
  ActionController get _$_PanelStateActionController => getActionController();
}

abstract class _PanelState with Store, Entity {
  @override
  late String id;

  @observable
  double size = 400.0;

  @observable
  bool isExpanded = true;

  @action
  void toggleExpanded() {
    isExpanded = !isExpanded;
  }
}

class PanelTabsState extends _PanelTabsState with _$PanelTabsState, Entity {
  @override
  String id;

  PanelTabsState(this.id);

  @override
  ActionController get _$_PanelTabsStateActionController =>
      getActionController();
}

abstract class _PanelTabsState with Store {
  @observable
  int selectedIndex = 0;

  @action
  void setSelectedIndex(int index) {
    selectedIndex = index;
  }
}

class DetailsPanelState extends _DetailsPanelState
    with _$DetailsPanelState, Entity {
  @override
  String id = "/UI/DetailsPanelState";

  @override
  ActionController get _$_DetailsPanelStateActionController =>
      getActionController();
}

abstract class _DetailsPanelState with Store {
  @observable
  PanelTabsState panelTabsState =
      PanelTabsState("/UI/DetailsPanelState/panelTabsState/PanelTabsState");

  @observable
  double height = 200;

  @observable
  MIDIClipModel midiClipModel = MIDIClipModel();

  @action
  void updateHeight(double deltaY) {
    height -= deltaY;
    height = Math.max(height, 200);
  }
}
