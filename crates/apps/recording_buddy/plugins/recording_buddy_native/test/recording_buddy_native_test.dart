import 'dart:ffi';

import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:recording_buddy_native/recording_buddy_native.dart';

void main() {
  const MethodChannel channel = MethodChannel('recording_buddy_native');

  TestWidgetsFlutterBinding.ensureInitialized();

  setUp(() {
    channel.setMockMethodCallHandler((MethodCall methodCall) async {
      return '42';
    });
  });

  tearDown(() {
    channel.setMockMethodCallHandler(null);
  });

  test('getPlatformVersion', () async {
    expect(await RecordingBuddyNative.platformVersion, '42');
  });

  // test('initialize_recording_buddy', () async {
  //   var fun = DynamicLibrary
  //       .process()
  //       .lookupFunction<Void Function(), void Function()>("initialize_recording_buddy");
  //   fun.call();
  // });
}
