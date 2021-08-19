
import 'dart:async';

import 'package:flutter/services.dart';

class RecordingBuddyNative {
  static const MethodChannel _channel =
      const MethodChannel('recording_buddy_native');

  static Future<String?> get platformVersion async {
    final String? version = await _channel.invokeMethod('getPlatformVersion');
    return version;
  }
}
