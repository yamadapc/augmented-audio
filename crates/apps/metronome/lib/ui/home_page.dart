import 'dart:ffi';

import 'package:flutter/cupertino.dart';

import './tabs/history/history_page_tab.dart';
import './tabs/main_tab.dart';
import '../bridge_generated.dart';
import '../logger.dart';
import '../modules/database.dart';
import '../modules/history/history_controller.dart';
import '../modules/state/history_state_controller.dart';
import '../modules/state/history_state_model.dart';
import '../modules/state/metronome_state_controller.dart';
import '../modules/state/metronome_state_model.dart';

Metronome buildMetronome() {
  var metronome = MetronomeImpl(DynamicLibrary.executable());
  metronome.initialize();
  return metronome;
}

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
    // var performance = FirebasePerformance.instance;
    // var trace = performance.newTrace("init-sequence");
    // trace.start();

    logger.i("Initializing metronome bridge");
    metronome = buildMetronome();

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

        // trace.stop();
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
    return HomePageContents(
        metronomeStateController: metronomeStateController,
        historyStateController: historyStateController);
  }
}

class HomePageContents extends StatelessWidget {
  const HomePageContents({
    Key? key,
    required this.metronomeStateController,
    required this.historyStateController,
  }) : super(key: key);

  final MetronomeStateController? metronomeStateController;
  final HistoryStateController? historyStateController;

  @override
  Widget build(BuildContext context) {
    if (metronomeStateController == null) {
      return const Center(child: Text("Loading..."));
    }

    return CupertinoTabScaffold(
      tabBar: CupertinoTabBar(items: const [
        BottomNavigationBarItem(
            icon: Icon(CupertinoIcons.play_arrow_solid), label: "Metronome"),
        BottomNavigationBarItem(
            icon: Icon(CupertinoIcons.book), label: "History")
      ]),
      tabBuilder: (context, index) {
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
