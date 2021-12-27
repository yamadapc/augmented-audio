import 'dart:ffi';

import 'package:flutter/material.dart';
import 'package:metronome/bridge_generated.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Metronome',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const MyHomePage(title: 'Metronome'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({Key? key, required this.title}) : super(key: key);

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  bool isPlaying = true;
  double volume = 0.3;
  double tempo = 120.0;

  late Metronome metronome;

  @override
  void initState() {
    metronome = Metronome(DynamicLibrary.executable());
    metronome.initialize();
  }

  @override
  void deactivate() {
    metronome.deinitialize();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Padding(
        padding: const EdgeInsets.all(8.0),
        child: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: <Widget>[
              buildSlider(
                  label: "Tempo ${tempo.toStringAsFixed(0)}",
                  min: 30.0,
                  max: 250.0,
                  value: tempo,
                  callback: onTempoChanged),
              buildSlider(
                  label: "Volume", value: volume, callback: onVolumeChanged),
              TextButton(
                  onPressed: onStartStopPressed, child: Text("Start/Stop"))
            ],
          ),
        ),
      ),
    );
  }

  Row buildSlider(
      {required String label,
      required double value,
      required callback,
      double min = 0.0,
      double max = 1.0}) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.center,
      children: [
        SizedBox(width: 70, child: Text(label)),
        Expanded(
          child: Slider(
              label: label,
              min: min,
              max: max,
              value: value,
              onChanged: callback),
        ),
      ],
    );
  }

  void onStartStopPressed() {
    metronome.setIsPlaying(value: !isPlaying);
    setState(() {
      isPlaying = !isPlaying;
    });
  }

  void onVolumeChanged(double value) {
    metronome.setVolume(value: value);
    setState(() {
      volume = value;
    });
  }

  void onTempoChanged(double value) {
    metronome.setTempo(value: value);
    setState(() {
      tempo = value;
    });
  }
}
