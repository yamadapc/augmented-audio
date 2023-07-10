import 'package:flutter/cupertino.dart';
import 'package:flutter_local_notifications/flutter_local_notifications.dart';
import 'package:flutter_timezone/flutter_timezone.dart';
import 'package:timezone/data/latest_10y.dart';
import 'package:timezone/timezone.dart';

class GoalPanel extends StatelessWidget {
  final FlutterLocalNotificationsPlugin flutterLocalNotificationsPlugin =
      FlutterLocalNotificationsPlugin();

  GoalPanel({super.key}) {
    flutterLocalNotificationsPlugin.initialize(
      const InitializationSettings(
        macOS: DarwinInitializationSettings(),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return CupertinoButton(
      padding: EdgeInsets.zero,
      minSize: 4,
      child: const Text("Set-up practice goal"),
      onPressed: () async {
        initializeTimeZones();
        final String timeZoneName = await FlutterTimezone.getLocalTimezone();
        final Location location = getLocation(timeZoneName);
        flutterLocalNotificationsPlugin.zonedSchedule(
          0,
          "Practice goal reminder",
          "You still need practice hours today to meet your practice goal",
          TZDateTime.from(
            DateTime.now().add(const Duration(seconds: 2)),
            location,
          ),
          const NotificationDetails(
            macOS: DarwinNotificationDetails(
              presentAlert: true,
              presentBadge: false,
              presentSound: false,
              badgeNumber: 0,
            ),
          ),
          uiLocalNotificationDateInterpretation:
              UILocalNotificationDateInterpretation.absoluteTime,
        );
      },
    );
  }
}
