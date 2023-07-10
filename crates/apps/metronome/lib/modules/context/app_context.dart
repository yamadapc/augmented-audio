import 'package:flutter/cupertino.dart';
import 'package:metronome/modules/analytics/analytics.dart';

class AppContext extends InheritedWidget {
  final Analytics analytics;

  const AppContext({
    super.key,
    required this.analytics,
    required super.child,
  });

  static AppContext of(BuildContext context) {
    return context.dependOnInheritedWidgetOfExactType<AppContext>()!;
  }

  @override
  bool updateShouldNotify(AppContext oldWidget) => false;
}
