void main() {}
// import 'package:flutter/cupertino.dart';
// import 'package:flutter_test/flutter_test.dart';
// import 'package:metronome/logger.dart';
// import 'package:metronome/ui/home_page.dart';
//
// import 'mock_metronome.dart';
//
// void main() async {
//   var env = await buildTestEnvironment();
//
//   testWidgets("Metronome light mode golden test", (tester) async {
//     // Render main page
//     logger.i("Ready to render");
//     await tester.pumpWidget(CupertinoApp(
//         title: 'Metronome',
//         theme: const CupertinoThemeData(brightness: Brightness.light),
//         home: HomePageContents(
//             metronomeStateController: env.metronomeStateController,
//             historyStateController: env.historyStateController)));
//     logger.i("Pumped widget");
//     await expectLater(find.byType(HomePageContents),
//         matchesGoldenFile('home-page-contents.png'));
//   });
//
//   testWidgets("Metronome dark mode golden test", (tester) async {
//     // Render main page
//     logger.i("Ready to render");
//     await tester.pumpWidget(CupertinoApp(
//         title: 'Metronome',
//         theme: const CupertinoThemeData(brightness: Brightness.dark),
//         home: HomePageContents(
//             metronomeStateController: env.metronomeStateController,
//             historyStateController: env.historyStateController)));
//     logger.i("Pumped widget");
//     await expectLater(find.byType(HomePageContents),
//         matchesGoldenFile('home-page-contents-dark.png'));
//   });
// }
