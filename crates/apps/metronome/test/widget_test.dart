import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_platform_widgets/flutter_platform_widgets.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:metronome/logger.dart';
import 'package:metronome/ui/home_page.dart';

import 'mock_metronome.dart';

void main() async {
  final env = await buildTestEnvironment();

  Future<void> pump({
    required WidgetTester tester,
    required TargetPlatform platform,
    required Brightness brightness,
    required Widget widget,
  }) async {
    await tester.pumpWidget(
      PlatformProvider(
        initialPlatform: platform,
        builder: (_) => PlatformApp(
          title: 'Metronome',
          cupertino: (_, __) => CupertinoAppData(
            theme: CupertinoThemeData(brightness: brightness),
          ),
          material: (_, __) => MaterialAppData(
            theme: brightness == Brightness.light
                ? ThemeData.light()
                : ThemeData.dark(),
          ),
          home: widget,
        ),
      ),
    );
  }

  testWidgets("Metronome light mode golden test cupertino", (tester) async {
    // Render main page
    logger.i("Ready to render");
    await pump(
      tester: tester,
      brightness: Brightness.light,
      platform: TargetPlatform.iOS,
      widget: HomePageContents(
        metronomeStateController: env.metronomeStateController,
        historyStateController: env.historyStateController,
      ),
    );

    logger.i("Pumped widget");
    await expectLater(
      find.byType(HomePageContents),
      matchesGoldenFile('home-page-contents.png'),
    );
  });

  testWidgets("Metronome dark mode golden test cupertino", (tester) async {
    // Render main page
    logger.i("Ready to render");
    await pump(
      tester: tester,
      brightness: Brightness.dark,
      platform: TargetPlatform.iOS,
      widget: HomePageContents(
        metronomeStateController: env.metronomeStateController,
        historyStateController: env.historyStateController,
      ),
    );
    logger.i("Pumped widget");
    await expectLater(
      find.byType(HomePageContents),
      matchesGoldenFile('home-page-contents-dark.png'),
    );
  });

  // testWidgets("Metronome light mode golden test material", (tester) async {
  //   // Render main page
  //   logger.i("Ready to render");
  //   await pump(
  //     tester: tester,
  //     brightness: Brightness.light,
  //     platform: TargetPlatform.android,
  //     widget: HomePageContents(
  //       metronomeStateController: env.metronomeStateController,
  //       historyStateController: env.historyStateController,
  //     ),
  //   );
  //   logger.i("Pumped widget");
  //   await expectLater(
  //     find.byType(HomePageContents),
  //     matchesGoldenFile('home-page-contents-android-light.png'),
  //   );
  // });
  //
  // testWidgets("Metronome dark mode golden test material", (tester) async {
  //   // Render main page
  //   logger.i("Ready to render");
  //   await pump(
  //     tester: tester,
  //     brightness: Brightness.dark,
  //     platform: TargetPlatform.android,
  //     widget: HomePageContents(
  //       metronomeStateController: env.metronomeStateController,
  //       historyStateController: env.historyStateController,
  //     ),
  //   );
  //   logger.i("Pumped widget");
  //   await expectLater(
  //     find.byType(HomePageContents),
  //     matchesGoldenFile('home-page-contents-android-dark.png'),
  //   );
  // });
}
