import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/ui/examples.dart';

import 'common/tabs.dart';
import 'main_content_layout/bottom_panel.dart';
import 'main_content_layout/debug/debug_view.dart';
import 'main_content_layout/header.dart';
import 'main_content_layout/sidebar.dart';
import 'main_content_layout/tracks_view.dart';

class MainContentLayout extends StatelessWidget {
  const MainContentLayout({Key? key, required this.title}) : super(key: key);

  final String title;

  @override
  Widget build(BuildContext context) {
    const tracksView = TracksView();

    var contentTabs = [
      ConcretePanelTab(0, "Tracks", tracksView),
      ConcretePanelTab(1, "Debug", const DebugView()),
      ConcretePanelTab(2, "Storybook", const DawStorybook()),
    ];
    var content = Expanded(child: PanelTabsView(tabs: contentTabs));

    // TODO - review random repaint boundaries
    return Scaffold(
        backgroundColor: const Color.fromRGBO(35, 35, 38, 1.0),
        body: Column(children: [
          const Header(),
          Expanded(
              child: Row(
            children: [
              const RepaintBoundary(child: Sidebar()),
              content,
            ],
          )),
          RepaintBoundary(child: BottomPanel()),
        ]));
  }
}
