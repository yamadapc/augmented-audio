import 'package:flutter/cupertino.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_local_notifications/flutter_local_notifications.dart';
import 'package:flutter_native_timezone/flutter_native_timezone.dart';
import 'package:timezone/data/latest_10y.dart';
import 'package:timezone/timezone.dart';

class GoalPanel extends StatelessWidget {
  FlutterLocalNotificationsPlugin flutterLocalNotificationsPlugin =
      FlutterLocalNotificationsPlugin();

  GoalPanel({Key? key}) : super(key: key) {
    flutterLocalNotificationsPlugin.initialize(const InitializationSettings(
        macOS: MacOSInitializationSettings(
      requestAlertPermission: true,
    )));
  }

  @override
  Widget build(BuildContext context) {
    return CupertinoButton(
        padding: EdgeInsets.zero,
        minSize: 4,
        child: const Text("Set-up practice goal"),
        onPressed: () async {
          initializeTimeZones();
          final String timeZoneName =
              await FlutterNativeTimezone.getLocalTimezone();
          final Location location = getLocation(timeZoneName);
          flutterLocalNotificationsPlugin.zonedSchedule(
              0,
              "Practice goal reminder",
              "You still need practice hours today to meet your practice goal",
              TZDateTime.from(
                  DateTime.now().add(const Duration(seconds: 2)), location),
              const NotificationDetails(
                  macOS: MacOSNotificationDetails(
                presentAlert: true,
                presentBadge: false,
                presentSound: false,
                badgeNumber: 0,
              )),
              androidAllowWhileIdle: false,
              uiLocalNotificationDateInterpretation:
                  UILocalNotificationDateInterpretation.absoluteTime);
          print("GOAL START");
        });
  }
}
