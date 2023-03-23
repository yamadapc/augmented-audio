import 'package:flutter/cupertino.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:metronome/modules/analytics/fake_analytics.dart';
import 'package:metronome/ui/app.dart';

import 'utils.dart';

void main() {
  final binding = IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  group('end-to-end test', () {
    testWidgets('wait for metronome to render', (tester) async {
      await tester.pumpWidget(App(analytics: FakeAnalytics()));
      await tester.pumpAndSettle();

      await waitFor(tester, find.byKey(const Key("MainPageTab")));
      await waitFor(tester, find.byKey(const Key("PlayButton")));
      await binding.convertFlutterSurfaceToImage();

      await tester.tap(find.byKey(const Key("PlayButton")));
      await tester.pumpAndSettle();
      await binding.convertFlutterSurfaceToImage();

      await tester.tap(find.byKey(const Key("PlayButton")));
      await tester.pumpAndSettle();

      await tester.tap(find.text("History"));
      await tester.pumpAndSettle();
      await waitFor(tester, find.byKey(const Key("HistoryPageTab")));
      await binding.convertFlutterSurfaceToImage();

      await tester.tap(find.text("Metronome"));
      await tester.pumpAndSettle();
      await waitFor(tester, find.byKey(const Key("MainPageTab")));
      await binding.convertFlutterSurfaceToImage();
    });
  });
}
