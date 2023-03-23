import 'package:metronome/modules/analytics/analytics.dart';

class FakeAnalytics implements Analytics {
  @override
  void logEvent({required String name}) {}

  @override
  void logScreenView({
    required String screenClass,
    required String screenName,
  }) {}
}
