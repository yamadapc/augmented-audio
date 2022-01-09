import 'dart:ffi';

import 'package:flutter/cupertino.dart';
import 'package:metronome/bridge_generated.dart';
import 'package:metronome/modules/state/metronome_state_controller.dart';
import 'package:mobx/mobx.dart';

import '../modules/state/metronome_state_model.dart';
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

  final MetronomeStateModel metronomeStateModel = MetronomeStateModel();

  late Metronome metronome;
  late MetronomeStateController metronomeStateController;

  @override
  void initState() {
    metronome = Metronome(DynamicLibrary.executable());
    metronomeStateController =
        MetronomeStateController(metronomeStateModel, metronome);
    metronome.initialize();
    metronomeStateController.setup();

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
            stateController: metronomeStateController,
          );
        } else {
          return const HistoryPageTab();
        }
      },
    );
  }
}
