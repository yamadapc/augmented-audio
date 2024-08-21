import 'dart:io' as io;
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_platform_widgets/flutter_platform_widgets.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:metronome/logger.dart';
import 'package:metronome/modules/context/app_context.dart';
import 'package:metronome/modules/database.dart';
import 'package:metronome/modules/history/history_controller.dart';
import 'package:metronome/modules/performance_metrics/metrics.dart';
import 'package:metronome/modules/state/history_state_controller.dart';
import 'package:metronome/modules/state/history_state_model.dart';
import 'package:metronome/modules/state/metronome_state_controller.dart';
import 'package:metronome/modules/state/metronome_state_model.dart';
import 'package:metronome/src/rust/api.dart';
import 'package:metronome/src/rust/frb_generated.dart';
import 'package:metronome/src/rust/internal/state.dart';
import 'package:metronome/ui/tabs/history/history_page_tab.dart';
import 'package:metronome/ui/tabs/main_tab.dart';
import 'package:path_provider/path_provider.dart';

Future<void> buildMetronome() async {
  // const name = "metronome";
  // final metronome = RustLib(
  //   io.Platform.isIOS || io.Platform.isMacOS
  //       ? DynamicLibrary.executable()
  //       : DynamicLibrary.open("lib$name.so"),
  // );
  await RustLib.init();

  final Directory applicationDocumentsDirectory =
      await getApplicationDocumentsDirectory();
  final options = InitializeOptions(
    assetsFilePath: io.Platform.isAndroid
        ? "${applicationDocumentsDirectory.parent.path}/files"
        : null,
  );
  initialize(options: options);
}

class HomePage extends StatefulWidget {
  const HomePage({super.key, required this.title});

  final String title;

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> with WidgetsBindingObserver {
  final HistoryStateModel historyStateModel = HistoryStateModel();
  final MetronomeStateModel metronomeStateModel = MetronomeStateModel();

  MetronomeStateController? metronomeStateController;
  HistoryStateController? historyStateController;

  void onError(dynamic err) {
    logger.e("ERROR: $err");
    showMacosAlertDialog(
      context: context,
      builder: (context) => Center(child: Text("ERROR: $err")),
    );
  }

  @override
  void initState() {
    final performance = getMetrics();
    final trace = performance.newTrace("init-sequence");
    trace.start();

    Future<void> runInitSequence() async {
      logger.i("Initializing metronome bridge");
      await buildMetronome();
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
          historyController,
          AppContext.of(context).analytics,
        );
        metronomeStateController?.setup();

        trace.stop();
      });

      streamErrors().listen((error) {
        onError(error);
      });
    }

    runInitSequence().catchError((err) {
      onError(err);
    });

    super.initState();
  }

  @override
  void deactivate() {
    metronomeStateController?.stop();
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
    super.key,
    required this.metronomeStateController,
    required this.historyStateController,
  });

  final MetronomeStateController? metronomeStateController;
  final HistoryStateController? historyStateController;

  @override
  Widget build(BuildContext context) {
    if (metronomeStateController == null) {
      return const Center(child: Text("Loading..."));
    }

    return PlatformTabScaffold(
      tabController: PlatformTabController(),
      items: [
        BottomNavigationBarItem(
          icon: Icon(PlatformIcons(context).playArrowSolid),
          label: "Metronome",
        ),
        BottomNavigationBarItem(
          icon: Icon(PlatformIcons(context).book),
          label: "History",
        ),
      ],
      bodyBuilder: (context, index) {
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
