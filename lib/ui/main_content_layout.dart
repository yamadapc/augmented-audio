import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/dev/storybook.dart';

import 'common/generic_sidebar.dart';
import 'common/tabs.dart';
import 'main_content_layout/bottom_panel.dart';
import 'main_content_layout/debug/debug_view.dart';
import 'main_content_layout/header.dart';
import 'main_content_layout/sidebar.dart';
import 'main_content_layout/tracks_view.dart';

StorybookState storybookState = StorybookState();

class MainContentLayout extends StatelessWidget {
  MainContentLayout({Key? key, required this.title}) : super(key: key);

  final String title;

  @override
  Widget build(BuildContext context) {
    const tracksView = TracksView();

    setupStories();
    StorybookView storybookView = StorybookView(storybookState: storybookState);

    var contentTabs = [
      ConcretePanelTab(0, "Tracks", tracksView),
      ConcretePanelTab(1, "Debug", const DebugView()),
      ConcretePanelTab(2, "Storybook", storybookView),
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

  void setupStories() {
    rootStory.stories.clear();
    rootStory.addStory(sidebarStory());
    storybookState.storybook = rootStory;
  }
}
