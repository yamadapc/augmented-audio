// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'ui_state.dart';

// **************************************************************************
// StoreGenerator
// **************************************************************************

// ignore_for_file: non_constant_identifier_names, unnecessary_brace_in_string_interps, unnecessary_lambdas, prefer_expression_function_bodies, lines_longer_than_80_chars, avoid_as, avoid_annotating_with_dynamic

mixin _$UIState on _UIState, Store {
  final _$sidebarStateAtom = Atom(name: '_UIState.sidebarState');

  @override
  SidebarState get sidebarState {
    _$sidebarStateAtom.reportRead();
    return super.sidebarState;
  }

  @override
  set sidebarState(SidebarState value) {
    _$sidebarStateAtom.reportWrite(value, super.sidebarState, () {
      super.sidebarState = value;
    });
  }

  @override
  String toString() {
    return '''
sidebarState: ${sidebarState}
    ''';
  }
}

mixin _$SidebarState on _SidebarState, Store {
  final _$panelStateAtom = Atom(name: '_SidebarState.panelState');

  @override
  PanelState get panelState {
    _$panelStateAtom.reportRead();
    return super.panelState;
  }

  @override
  set panelState(PanelState value) {
    _$panelStateAtom.reportWrite(value, super.panelState, () {
      super.panelState = value;
    });
  }

  @override
  String toString() {
    return '''
panelState: ${panelState}
    ''';
  }
}

mixin _$PanelState on _PanelState, Store {
  final _$sizeAtom = Atom(name: '_PanelState.size');

  @override
  double get size {
    _$sizeAtom.reportRead();
    return super.size;
  }

  @override
  set size(double value) {
    _$sizeAtom.reportWrite(value, super.size, () {
      super.size = value;
    });
  }

  final _$isExpandedAtom = Atom(name: '_PanelState.isExpanded');

  @override
  bool get isExpanded {
    _$isExpandedAtom.reportRead();
    return super.isExpanded;
  }

  @override
  set isExpanded(bool value) {
    _$isExpandedAtom.reportWrite(value, super.isExpanded, () {
      super.isExpanded = value;
    });
  }

  final _$_PanelStateActionController = ActionController(name: '_PanelState');

  @override
  void toggleExpanded() {
    final _$actionInfo = _$_PanelStateActionController.startAction(
        name: '_PanelState.toggleExpanded');
    try {
      return super.toggleExpanded();
    } finally {
      _$_PanelStateActionController.endAction(_$actionInfo);
    }
  }

  @override
  String toString() {
    return '''
size: ${size},
isExpanded: ${isExpanded}
    ''';
  }
}

mixin _$PanelTabsState on _PanelTabsState, Store {
  final _$selectedIndexAtom = Atom(name: '_PanelTabsState.selectedIndex');

  @override
  int get selectedIndex {
    _$selectedIndexAtom.reportRead();
    return super.selectedIndex;
  }

  @override
  set selectedIndex(int value) {
    _$selectedIndexAtom.reportWrite(value, super.selectedIndex, () {
      super.selectedIndex = value;
    });
  }

  final _$_PanelTabsStateActionController =
      ActionController(name: '_PanelTabsState');

  @override
  void setSelectedIndex(int index) {
    final _$actionInfo = _$_PanelTabsStateActionController.startAction(
        name: '_PanelTabsState.setSelectedIndex');
    try {
      return super.setSelectedIndex(index);
    } finally {
      _$_PanelTabsStateActionController.endAction(_$actionInfo);
    }
  }

  @override
  String toString() {
    return '''
selectedIndex: ${selectedIndex}
    ''';
  }
}
