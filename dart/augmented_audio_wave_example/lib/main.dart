import 'dart:ffi';

import 'package:augmented_audio_wave_example/bridge_generated.dart';
import 'package:flutter/material.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  var nativeApi = AugmentedAudioWaveExampleImpl(DynamicLibrary.executable());
  var value = 0;

  @override
  void initState() {
    super.initState();

    nativeApi.add(left: 10, right: 20).then((value) => {
      setState(() {
        this.value = value;
      })
    });
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: Text("Hello world $value"),
    );
  }
}
