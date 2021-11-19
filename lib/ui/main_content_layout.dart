import 'package:flutter/material.dart';

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
      PanelTab(0, "Tracks", tracksView),
      PanelTab(1, "Debug", DebugView()),
    ];
    var content = Expanded(child: PanelTabsView(tabs: contentTabs));

    return Scaffold(
        backgroundColor: const Color.fromRGBO(35, 35, 38, 1.0),
        body: Column(children: [
          const Header(),
          Expanded(
              child: Row(
            children: [
              const Sidebar(),
              content,
            ],
          )),
          const BottomPanel(),
        ]));
  }
}
