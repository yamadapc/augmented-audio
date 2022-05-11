import 'package:flutter/cupertino.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:metronome/logger.dart';
import 'package:metronome/modules/database.dart';
import 'package:metronome/modules/history/history_controller.dart';
import 'package:metronome/modules/state/history_state_controller.dart';
import 'package:metronome/modules/state/history_state_model.dart';
import 'package:metronome/modules/state/metronome_state_controller.dart';
import 'package:metronome/modules/state/metronome_state_model.dart';
import 'package:metronome/ui/home_page.dart';

import 'mock_metronome.dart';

void main() async {
  // Set-up mocked environment
  logger.i("Setting-up mock environment");
  var model = MetronomeStateModel();
  var metronome = MockMetronomeLib();
  var inMemoryDb = await buildInMemoryDatabase();
  var sessionDao = inMemoryDb.sessionDao;
  var historyStateModel = HistoryStateModel();
  var historyStateController =
      HistoryStateController(sessionDao, historyStateModel);
  var historyStartStopHandler =
      HistoryStartStopHandler(sessionDao, model, historyStateController);
  var metronomeStateController =
      MetronomeStateController(model, metronome, historyStartStopHandler);

  testWidgets("Metronome light mode golden test", (tester) async {
    // Render main page
    logger.i("Ready to render");
    await tester.pumpWidget(CupertinoApp(
        title: 'Metronome',
        theme: const CupertinoThemeData(brightness: Brightness.light),
        home: HomePageContents(
            metronomeStateController: metronomeStateController,
            historyStateController: historyStateController)));
    logger.i("Pumped widget");
    await expectLater(find.byType(HomePageContents),
        matchesGoldenFile('home-page-contents.png'));
  });

  testWidgets("Metronome dark mode golden test", (tester) async {
    // Render main page
    logger.i("Ready to render");
    await tester.pumpWidget(CupertinoApp(
        title: 'Metronome',
        theme: const CupertinoThemeData(brightness: Brightness.dark),
        home: HomePageContents(
            metronomeStateController: metronomeStateController,
            historyStateController: historyStateController)));
    logger.i("Pumped widget");
    await expectLater(find.byType(HomePageContents),
        matchesGoldenFile('home-page-contents-dark.png'));
  });
}
