import 'package:flutter/material.dart';

import 'main_content_layout/bottom_panel.dart';
import 'main_content_layout/header.dart';
import 'main_content_layout/sidebar.dart';
import 'main_content_layout/tracks_view.dart';

class MainContentLayout extends StatelessWidget {
  const MainContentLayout({Key? key, required this.title}) : super(key: key);

  final String title;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        backgroundColor: const Color.fromRGBO(35, 35, 38, 1.0),
        body: Column(children: [
          const Header(),
          Expanded(
              child: Row(
            children: const [
              Sidebar(),
              Expanded(child: TracksView()),
            ],
          )),
          const BottomPanel(),
        ]));
  }
}
