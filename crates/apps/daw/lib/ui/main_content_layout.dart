import 'package:augmented_midi_editor/midi_editor/standalone_midi_editor.dart';
import 'package:flutter/material.dart';

import './common/status_bar.dart';
import './common/tabs.dart';
import './main_content_layout/bottom_panel.dart';
import './main_content_layout/header.dart';
import './main_content_layout/sidebar.dart';
import './main_content_layout/tracks_view.dart';
import '../state/project.dart';
import '../state/ui_state.dart';
import '../ui/examples.dart';
import '../ui/settings/settings_view.dart';
import '../ui/vst_development/vst_development.dart';

class MainContentLayout extends StatelessWidget {
  final Project project;
  final UIState uiState;

  const MainContentLayout(
      {Key? key,
      required this.title,
      required this.project,
      required this.uiState})
      : super(key: key);

  final String title;

  @override
  Widget build(BuildContext context) {
    var tracksView = TracksView(tracksList: project.tracksList);
    var contentTabs = [
      ConcretePanelTab(0, "Tracks", tracksView),
      ConcretePanelTab(1, "Storybook", const DawStorybook()),
      ConcretePanelTab(2, "Settings", const SettingsView()),
      ConcretePanelTab(3, "VST Development", const VSTDevelopmentView()),
      ConcretePanelTab(4, "MIDI Editor", StandaloneMIDIEditorView()),
    ];
    var content = Expanded(
        child: PanelTabsView(
            tabs: contentTabs, panelTabsState: uiState.mainContentTabsState));

    // TODO - review random repaint boundaries
    return Scaffold(
        backgroundColor: const Color.fromRGBO(35, 35, 38, 1.0),
        body: Column(children: [
          const Header(),
          Expanded(
              child: Row(
            children: [
              RepaintBoundary(
                  child: Sidebar(sidebarState: uiState.sidebarState)),
              content,
            ],
          )),
          RepaintBoundary(
              child: BottomPanel(detailsPanelState: uiState.detailsPanelState)),
          const StatusBar(),
        ]));
  }
}
