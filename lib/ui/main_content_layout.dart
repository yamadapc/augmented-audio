import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/state/project.dart';
import 'package:flutter_daw_mock_ui/state/ui_state.dart';
import 'package:flutter_daw_mock_ui/ui/examples.dart';
import 'package:flutter_daw_mock_ui/ui/settings/settings_view.dart';

import 'common/status_bar.dart';
import 'common/tabs.dart';
import 'main_content_layout/bottom_panel.dart';
import 'main_content_layout/header.dart';
import 'main_content_layout/sidebar.dart';
import 'main_content_layout/tracks_view.dart';

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
          const RepaintBoundary(child: BottomPanel()),
          const StatusBar(),
        ]));
  }
}
