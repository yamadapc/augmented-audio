import 'dart:ffi';

import 'package:flutter/cupertino.dart';
import 'package:metronome/bridge_generated.dart';
import 'package:metronome/logger.dart';
import 'package:metronome/modules/database.dart';
import 'package:metronome/modules/history/history_controller.dart';
import 'package:metronome/modules/state/history_state_controller.dart';
import 'package:metronome/modules/state/history_state_model.dart';
import 'package:metronome/modules/state/metronome_state_controller.dart';

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
  final HistoryStateModel historyStateModel = HistoryStateModel();
  final MetronomeStateModel metronomeStateModel = MetronomeStateModel();

  late Metronome metronome;
  MetronomeStateController? metronomeStateController;
  HistoryStateController? historyStateController;

  @override
  void initState() {
    logger.i("Initializing metronome bridge");
    metronome = Metronome(DynamicLibrary.executable());
    metronome.initialize();

    logger.i("Opening SQLite database");
    var databasePromise = buildDatabase();
    databasePromise.then((database) {
      logger.i("Setting-up controllers");

      historyStateController =
          HistoryStateController(database.sessionDao, historyStateModel);
      var historyController = HistoryStartStopHandler(
          database.sessionDao, metronomeStateModel, historyStateController!);

      setState(() {
        metronomeStateController = MetronomeStateController(
            metronomeStateModel, metronome, historyController);
        metronomeStateController?.setup();
      });
    }).catchError((err) {
      logger.e("ERROR: $err");
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
    if (metronomeStateController == null) {
      return Center(child: Text("Loading..."));
    }

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
            stateController: metronomeStateController!,
          );
        } else {
          return HistoryPageTab(stateController: historyStateController!);
        }
      },
    );
  }
}
