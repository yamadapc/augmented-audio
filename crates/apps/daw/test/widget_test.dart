// This is a basic Flutter widget test.
//
// To perform an interaction with a widget in your test, use the WidgetTester
// utility that Flutter provides. For example, you can send tap and scroll
// gestures. You can also use WidgetTester to find child widgets in the widget
// tree, read text, and verify that the values of widget properties are correct.

import 'dart:ui';

import 'package:flutter/widgets.dart';
import 'package:flutter_daw_mock_ui/ui/daw_app.dart';
import 'package:flutter_test/flutter_test.dart';
// import 'package:golden_toolkit/golden_toolkit.dart';

void main() {
  testWidgets('Trigger frame on App', (WidgetTester tester) async {
    // Build our app and trigger a frame.
    await tester.pumpWidget(const DawApp());
  });

/*
  testGoldens("DAW App", (tester) async {
    await loadAppFonts();

    var builder = GoldenBuilder.column();
    builder.addScenario(
        "App", const SizedBox(width: 1920, height: 1080, child: DawApp()));
    var widget = builder.build();
    await tester.pumpWidgetBuilder(widget, surfaceSize: const Size(1920, 1180));
    await screenMatchesGolden(tester, "golden_daw_app", autoHeight: true);
  });
*/
}
