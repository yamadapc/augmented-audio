// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'settings_view.dart';

// **************************************************************************
// StoreGenerator
// **************************************************************************

// ignore_for_file: non_constant_identifier_names, unnecessary_brace_in_string_interps, unnecessary_lambdas, prefer_expression_function_bodies, lines_longer_than_80_chars, avoid_as, avoid_annotating_with_dynamic

mixin _$SettingsState on _SettingsState, Store {
  final _$selectedTabAtom = Atom(name: '_SettingsState.selectedTab');

  @override
  String get selectedTab {
    _$selectedTabAtom.reportRead();
    return super.selectedTab;
  }

  @override
  set selectedTab(String value) {
    _$selectedTabAtom.reportWrite(value, super.selectedTab, () {
      super.selectedTab = value;
    });
  }

  final _$_SettingsStateActionController =
      ActionController(name: '_SettingsState');

  @override
  void setSelectedTab(String tab) {
    final _$actionInfo = _$_SettingsStateActionController.startAction(
        name: '_SettingsState.setSelectedTab');
    try {
      return super.setSelectedTab(tab);
    } finally {
      _$_SettingsStateActionController.endAction(_$actionInfo);
    }
  }

  @override
  String toString() {
    return '''
selectedTab: ${selectedTab}
    ''';
  }
}
