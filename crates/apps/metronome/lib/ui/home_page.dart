import 'dart:ffi';

import 'package:flutter/cupertino.dart';
import 'package:metronome/bridge_generated.dart';
import 'package:metronome/logger.dart';
import 'package:metronome/modules/database.dart';
import 'package:metronome/modules/history/history_controller.dart';
import 'package:metronome/modules/performance_metrics/metrics.dart';
import 'package:metronome/modules/state/history_state_controller.dart';
import 'package:metronome/modules/state/history_state_model.dart';
import 'package:metronome/modules/state/metronome_state_controller.dart';
import 'package:metronome/modules/state/metronome_state_model.dart';
import 'package:metronome/ui/tabs/history/history_page_tab.dart';
import 'package:metronome/ui/tabs/main_tab.dart';

Metronome buildMetronome() {
  final metronome = MetronomeImpl(DynamicLibrary.executable());
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
    final performance = getMetrics();
    final trace = performance.newTrace("init-sequence");
    trace.start();

    logger.i("Initializing metronome bridge");
    metronome = buildMetronome();

    final databasePromise = buildDatabase();
    databasePromise.then((database) {
      logger.i("Setting-up controllers");

      historyStateController =
          HistoryStateController(database.sessionDao, historyStateModel);
      final historyController = HistoryStartStopHandler(
        database.sessionDao,
        metronomeStateModel,
        historyStateController!,
      );

      setState(() {
        metronomeStateController = MetronomeStateController(
          metronomeStateModel,
          metronome,
          historyController,
        );
        metronomeStateController?.setup();

        trace.stop();
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
      historyStateController: historyStateController,
    );
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
      tabBar: CupertinoTabBar(
        items: const [
          BottomNavigationBarItem(
            icon: Icon(CupertinoIcons.play_arrow_solid),
            label: "Metronome",
          ),
          BottomNavigationBarItem(
            icon: Icon(CupertinoIcons.book),
            label: "History",
          )
        ],
      ),
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
