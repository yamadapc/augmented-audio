import 'package:flutter/material.dart';

import 'MainContentLayout/BottomPanel.dart';
import 'MainContentLayout/Header.dart';
import 'MainContentLayout/Sidebar.dart';
import 'MainContentLayout/TracksView.dart';

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
