import 'dart:ffi';

import 'package:flutter/cupertino.dart';
import 'package:metronome/bridge_generated.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return const CupertinoApp(
      title: 'Metronome',
      theme: CupertinoThemeData(),
      home: MyHomePage(title: 'Metronome'),
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
  double volume = 0.075;
  double tempo = 120.0;

  late Metronome metronome;

  @override
  void initState() {
    metronome = Metronome(DynamicLibrary.executable());
    metronome.initialize();
    super.initState();
  }

  @override
  void deactivate() {
    metronome.deinitialize();
    super.deactivate();
  }

  @override
  Widget build(BuildContext context) {
    return CupertinoPageScaffold(
      child: Padding(
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
              CupertinoButton(
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
        SizedBox(width: 90, child: Text(label)),
        Expanded(
          child: CupertinoSlider(
              min: min, max: max, value: value, onChanged: callback),
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
    metronome.setVolume(value: value * 4.0);
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
