import 'dart:ffi';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:graphx/graphx.dart';
import 'package:metronome/bridge_generated.dart';
import 'package:mobx/mobx.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return const CupertinoApp(
      title: 'Metronome',
      theme: CupertinoThemeData(
          scaffoldBackgroundColor: Color.fromRGBO(0, 0, 0, 0.0),
          brightness: Brightness.dark),
      home: HomePage(title: 'Metronome'),
    );
  }
}

class HomePage extends StatefulWidget {
  const HomePage({Key? key, required this.title}) : super(key: key);

  final String title;

  @override
  State<HomePage> createState() => _HomePageState();
}

var labelTextStyle = TextStyle(color: CupertinoColors.white.withOpacity(0.6));

class _HomePageState extends State<HomePage> {
  bool isPlaying = false;
  double volume = 0.075;
  double tempo = 120.0;
  Observable<double> playhead = Observable(0.0);

  late Metronome metronome;

  @override
  void initState() {
    metronome = Metronome(DynamicLibrary.executable());
    metronome.initialize();
    metronome.getPlayhead().forEach((element) {
      if (playhead.value == element) {
        return;
      }

      runInAction(() {
        playhead.value = element;
      });
    });

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
      child: Container(
        decoration: const BoxDecoration(
            gradient: LinearGradient(
                begin: Alignment.topCenter,
                end: Alignment.bottomCenter,
                colors: [
              Color.fromRGBO(35, 35, 35, 1.0),
              Color.fromRGBO(20, 20, 20, 1.0),
            ])),
        child: Padding(
          padding: const EdgeInsets.all(8.0),
          child: Column(
            children: [
              SizedBox(
                height: 70,
                width: double.infinity,
                child: SceneBuilderWidget(
                  builder: () => SceneController(
                    config: SceneConfig(
                      autoUpdateRender: false,
                      painterWillChange: false,
                    ),
                    back: MetronomeSceneBack(playhead),
                  ),
                  child: null,
                ),
              ),
              Expanded(
                child: Center(
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: <Widget>[
                      Column(children: [
                        Text("tempo",
                            textScaleFactor: .8, style: labelTextStyle),
                        Text(tempo.toStringAsFixed(0), textScaleFactor: 2.0),
                        SizedBox(
                          width: double.infinity,
                          child: CupertinoSlider(
                              value: tempo,
                              onChanged: onTempoChanged,
                              min: 30,
                              max: 250),
                        )
                      ]),
                      buildSlider(
                          label: "volume",
                          value: volume,
                          callback: onVolumeChanged),
                      const Spacer(),
                      BottomRow(onStartStopPressed: onStartStopPressed)
                    ],
                  ),
                ),
              )
            ],
          ),
        ),
      ),
    );
  }

  Widget buildSlider(
      {required String label,
      required double value,
      required callback,
      double min = 0.0,
      double max = 1.0}) {
    return Column(
      children: [
        Text(label, style: labelTextStyle, textScaleFactor: 0.8),
        SizedBox(
            width: double.infinity,
            child: CupertinoSlider(
                min: min, max: max, value: value, onChanged: callback)),
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
    metronome.setVolume(value: value * 100.0);
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

class BottomRow extends StatelessWidget {
  VoidCallback onStartStopPressed;

  BottomRow({Key? key, required this.onStartStopPressed}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Row(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          Expanded(
            child: CupertinoButton(
                color: CupertinoColors.activeBlue,
                onPressed: onStartStopPressed,
                child: const Text("Start/Stop",
                    style: TextStyle(color: CupertinoColors.white))),
          )
        ]);
  }
}

class MetronomeSceneBack extends GSprite {
  Observable<double> playhead;
  Dispose? subscription;

  MetronomeSceneBack(this.playhead);

  @override
  void addedToStage() {
    subscription = playhead.observe((_) {
      stage!.scene.requestRender();
    });
  }

  @override
  void removedFromStage() {
    subscription?.call();
  }

  @override
  void paint(Canvas canvas) {
    var playheadValue = playhead.value;
    var playheadPrime = 1.0 - playheadValue % 1.0;

    var width = stage?.stageWidth ?? 100.0;
    var padding = 5.0;
    var rectWidth = (width - padding * 2) / 4;
    var left = 0.0;
    var top = rectWidth / 2.0;

    for (var i = 0; i < 4; i++) {
      var isTick = playheadValue % 4.0 >= i && playheadValue % 4.0 < (i + 1);
      var tickFactor = (isTick ? .4 : .0);

      var offset = Offset(left + rectWidth / 2.0, top);

      if (isTick) {
        Paint strokePaint = Paint();
        strokePaint.color = Color.fromRGBO(255, 255, 255, 1.0 * playheadPrime);
        var rect = Rect.fromCircle(center: offset, radius: rectWidth / 2.0 - 3);
        var rrect = RRect.fromRectAndRadius(rect, const Radius.circular(10.0));
        canvas.drawRRect(rrect, strokePaint);
      }

      Paint paint = Paint();
      var baseColor = CupertinoColors.activeBlue;
      paint.color =
          baseColor.withOpacity(0.4 + 1.2 * playheadPrime * tickFactor);
      var rect = Rect.fromCircle(center: offset, radius: rectWidth / 2.0 - 5);
      var rrect = RRect.fromRectAndRadius(rect, const Radius.circular(10.0));
      canvas.drawRRect(rrect, paint);
      left += rectWidth + padding;
    }
  }
}
