import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:metronome/main.dart' as app;

void main() {
  final binding = IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  group('end-to-end test', () {
    testWidgets('tap on the floating action button, verify counter',
        (tester) async {
      app.main();
      await tester.pumpAndSettle();

      // Verify the counter starts at 0.
      expect(find.text('Start'), findsOneWidget);
      await binding.convertFlutterSurfaceToImage();
      await binding.takeScreenshot('screenshot-1-init');

      final Finder start = find.text('Start');
      await tester.tap(start);
      await tester.pumpAndSettle();
      await binding.convertFlutterSurfaceToImage();
      await binding.takeScreenshot('screenshot-2-started');

      final Finder stop = find.text('Stop');
      await tester.tap(stop);
      await tester.pumpAndSettle();

      final Finder history = find.text("History");
      await tester.tap(history);
      await tester.pumpAndSettle();
      await binding.convertFlutterSurfaceToImage();
      await binding.takeScreenshot('screenshot-3-history');
    });
  });
}
