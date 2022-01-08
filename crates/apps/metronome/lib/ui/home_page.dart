import 'dart:ffi';

import 'package:flutter/cupertino.dart';
import 'package:metronome/bridge_generated.dart';
import 'package:mobx/mobx.dart';

import 'tabs/history_page_tab.dart';
import 'tabs/main_tab.dart';

class HomePage extends StatefulWidget {
  const HomePage({Key? key, required this.title}) : super(key: key);

  final String title;

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  Observable<bool> isPlaying = Observable(false);
  Observable<double> volume = Observable(0.3);
  Observable<double> tempo = Observable(120.0);
  Observable<double> playhead = Observable(0.0);

  late Metronome metronome;

  @override
  void initState() {
    metronome = Metronome(DynamicLibrary.executable());
    metronome.initialize();
    metronome.getPlayhead().forEach((element) {
      if (playhead.value == element) {
        return;
      }

      runInAction(() {
        playhead.value = element;
      });
    });

    super.initState();
  }

  @override
  void deactivate() {
    metronome.deinitialize();
    super.deactivate();
  }

  @override
  Widget build(BuildContext context) {
    return CupertinoTabScaffold(
      tabBar: CupertinoTabBar(items: const [
        BottomNavigationBarItem(
            icon: Icon(CupertinoIcons.play_arrow_solid), label: "Metronome"),
        BottomNavigationBarItem(
            icon: Icon(CupertinoIcons.book), label: "History")
      ]),
      tabBuilder: (_context, index) {
        if (index == 0) {
          return MainPageTab(
              playhead: playhead,
              tempo: tempo,
              metronome: metronome,
              volume: volume,
              isPlaying: isPlaying);
        } else {
          return const HistoryPageTab();
        }
      },
    );
  }
}
