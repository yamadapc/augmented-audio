abstract class Analytics {
  void logEvent({required String name});
  void logScreenView({
    required String screenClass,
    required String screenName,
  });
}
