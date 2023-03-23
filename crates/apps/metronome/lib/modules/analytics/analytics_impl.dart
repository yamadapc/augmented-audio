import 'package:firebase_analytics/firebase_analytics.dart';
import 'package:metronome/modules/analytics/analytics.dart';

class AnalyticsImpl implements Analytics {
  static final AnalyticsImpl instance = AnalyticsImpl();

  @override
  void logEvent({required String name}) {
    final analytics = FirebaseAnalytics.instance;
    analytics.logEvent(name: name);
  }

  @override
  void logScreenView({
    required String screenClass,
    required String screenName,
  }) {
    final analytics = FirebaseAnalytics.instance;
    analytics.logScreenView(
      screenClass: screenClass,
      screenName: screenName,
    );
  }
}
