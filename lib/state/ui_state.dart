import 'package:mobx/mobx.dart';

part 'ui_state.g.dart';

class UIState = _UIState with _$UIState;

abstract class _UIState with Store {
  @observable
  SidebarState sidebarState = SidebarState();
}

class SidebarState = _SidebarState with _$SidebarState;

abstract class _SidebarState with Store {
  @observable
  PanelState panelState = PanelState();
}

class PanelState = _PanelState with _$PanelState;

abstract class _PanelState with Store {
  @observable
  double size = 400.0;

  @observable
  bool isExpanded = true;

  @action
  void toggleExpanded() {
    isExpanded = !isExpanded;
  }
}

class PanelTabsState = _PanelTabsState with _$PanelTabsState;

abstract class _PanelTabsState with Store {
  @observable
  int selectedIndex = 0;

  @action
  void setSelectedIndex(int index) {
    selectedIndex = index;
  }
}
