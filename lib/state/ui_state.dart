import 'package:flutter_daw_mock_ui/ui/main_content_layout/midi_editor/midi_model.dart';
import 'package:mobx/mobx.dart';

part 'ui_state.g.dart';

class UIState = _UIState with _$UIState;

abstract class _UIState with Store {
  @observable
  SidebarState sidebarState = SidebarState();

  @observable
  PanelTabsState mainContentTabsState = PanelTabsState();

  @observable
  DetailsPanelState detailsPanelState = DetailsPanelState();
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

class DetailsPanelState = _DetailsPanelState with _$DetailsPanelState;

abstract class _DetailsPanelState with Store {
  @observable
  PanelTabsState panelTabsState = PanelTabsState();

  @observable
  double height = 200;

  @observable
  MIDIClipModel midiClipModel = MIDIClipModel();

  @action
  void updateHeight(double deltaY) {
    height -= deltaY;
  }
}
