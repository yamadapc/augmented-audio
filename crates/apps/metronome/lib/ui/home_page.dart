import 'dart:ffi';
import 'dart:io' as io;
import 'dart:io';

import 'package:flutter/cupertino.dart';
import 'package:metronome/bridge_generated.dart';
import 'package:metronome/logger.dart';
import 'package:metronome/modules/context/app_context.dart';
import 'package:metronome/modules/database.dart';
import 'package:metronome/modules/history/history_controller.dart';
import 'package:metronome/modules/performance_metrics/metrics.dart';
import 'package:metronome/modules/state/history_state_controller.dart';
import 'package:metronome/modules/state/history_state_model.dart';
import 'package:metronome/modules/state/metronome_state_controller.dart';
import 'package:metronome/modules/state/metronome_state_model.dart';
import 'package:metronome/ui/tabs/history/history_page_tab.dart';
import 'package:metronome/ui/tabs/main_tab.dart';
import 'package:path_provider/path_provider.dart';

Future<Metronome> buildMetronome() async {
  const name = "metronome";
  final metronome = MetronomeImpl(
    io.Platform.isIOS || io.Platform.isMacOS
        ? DynamicLibrary.executable()
        : DynamicLibrary.open("lib$name.so"),
  );

  final Directory applicationDocumentsDirectory =
      await getApplicationDocumentsDirectory();
  final options = InitializeOptions(
    assetsFilePath: io.Platform.isAndroid
        ? "${applicationDocumentsDirectory.parent.path}/files"
        : null,
  );
  metronome.initialize(options: options);
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

  Metronome? metronome;
  MetronomeStateController? metronomeStateController;
  HistoryStateController? historyStateController;

  @override
  void initState() {
    final performance = getMetrics();
    final trace = performance.newTrace("init-sequence");
    trace.start();

    Future<void> runInitSequence() async {
      logger.i("Initializing metronome bridge");
      final metronome = await buildMetronome();
      this.metronome = metronome;
      logger.i("Initializing database");
      final database = await buildDatabase();

      logger.i("Initializing controllers");
      historyStateController =
          HistoryStateController(database.sessionDao, historyStateModel);
      final historyController = HistoryStartStopHandler(
        database.sessionDao,
        metronomeStateModel,
        historyStateController!,
      );

      logger.i("Finishing init sequence");
      setState(() {
        metronomeStateController = MetronomeStateController(
          metronomeStateModel,
          metronome,
          historyController,
          AppContext.of(context).analytics,
        );
        metronomeStateController?.setup();

        trace.stop();
      });
    }

    runInitSequence().catchError((err) {
      logger.e("ERROR: $err");
    });

    super.initState();
  }

  @override
  void deactivate() {
    metronome?.deinitialize();
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
            key: const Key("MainPageTab"),
            stateController: metronomeStateController!,
          );
        } else {
          return HistoryPageTab(
            key: const Key("HistoryPageTab"),
            stateController: historyStateController!,
          );
        }
      },
    );
  }
}
