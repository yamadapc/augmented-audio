import 'package:flutter/cupertino.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:metronome/logger.dart';
import 'package:metronome/ui/controls/tempo_control/input_done_view.dart';
import 'package:mockito/annotations.dart';
import 'package:mockito/mockito.dart';

@GenerateNiceMocks(
  [MockSpec<FocusNode>()],
)
// ignore: unused_import, always_use_package_imports
import './input_done_view_test.mocks.dart';

void main() async {
  testWidgets("Input done view", (tester) async {
    // Render main page
    logger.i("Ready to render");
    await tester.pumpWidget(
      const CupertinoApp(
        title: 'Metronome',
        theme: CupertinoThemeData(brightness: Brightness.light),
        home: InputDoneView(),
      ),
    );
    logger.i("Pumped widget");
    await expectLater(
      find.byType(InputDoneView),
      matchesGoldenFile('input-done-view.png'),
    );
  });

  testWidgets("Input done view done can be clicked", (tester) async {
    final FocusNode mockFocusNode = MockFocusNode();
    await tester.pumpWidget(
      CupertinoApp(
        title: 'Metronome',
        theme: const CupertinoThemeData(brightness: Brightness.light),
        home: InputDoneView(targetFocusNode: mockFocusNode),
      ),
    );

    final Finder doneButton = find.byKey(
      const Key("ui.controls.tempo-control.input-done-view.done-button"),
    );
    expect(doneButton, findsOneWidget);
    await tester.tap(doneButton);
    await tester.pumpAndSettle();
    verify(mockFocusNode.requestFocus()).called(1);
  });
}
