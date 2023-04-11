import 'dart:io';

import 'package:flutter/cupertino.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:graphx/graphx.dart';
import 'package:metronome/modules/state/metronome_state_model.dart';
import 'package:metronome/ui/app.dart';
import 'package:metronome/ui/visualisation/scene.dart';

class ElapsedBeatsVisualisation extends StatelessWidget {
  const ElapsedBeatsVisualisation({
    Key? key,
    required this.model,
  }) : super(key: key);

  final MetronomeStateModel model;

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.only(left: 15, right: 15),
      width: 80,
      height: 30,
      decoration: BoxDecoration(
        border: Border.all(color: CupertinoColors.secondarySystemFill),
        borderRadius: BorderRadius.circular(Platform.isAndroid ? 1000 : 6.0),
        color: CupertinoColors.secondarySystemBackground
            .resolveFrom(context)
            .withOpacity(0.7),
      ),
      child: Center(
        child: Observer(
          builder: (_) {
            if (!model.isPlaying) {
              return const Text("0/0");
            }

            final beatsPerBar = model.beatsPerBar;
            final beat = Math.floor((model.playhead) % beatsPerBar) + 1;
            final bar = Math.floor(model.playhead / beatsPerBar);

            return Text(
              "${beat.toStringAsFixed(0)}/${bar.toStringAsFixed(0)}",
            );
          },
        ),
      ),
    );
  }
}

class ElapsedTimeVisualisation extends StatelessWidget {
  const ElapsedTimeVisualisation({
    Key? key,
    required this.model,
  }) : super(key: key);

  final MetronomeStateModel model;

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.only(left: 15, right: 15),
      width: 80,
      height: 30,
      decoration: BoxDecoration(
        border: Border.all(color: CupertinoColors.secondarySystemFill),
        borderRadius: BorderRadius.circular(Platform.isAndroid ? 10000 : 6.0),
        color: CupertinoColors.secondarySystemBackground
            .resolveFrom(context)
            .withOpacity(0.7),
      ),
      child: Center(
        child: Observer(
          builder: (_) {
            final elapsed = model.sessionState.duration;
            if (!model.isPlaying || elapsed.inSeconds < 0) {
              return const Text("0:00");
            }

            final minutes = elapsed.inMinutes;
            final seconds = elapsed.inSeconds % 60;
            final secondsStr = seconds.toString().padLeft(2, "0");

            return Text(
              "${minutes.toStringAsFixed(0)}:$secondsStr",
            );
          },
        ),
      ),
    );
  }
}

class Visualisation extends StatelessWidget {
  const Visualisation({
    Key? key,
    required this.model,
  }) : super(key: key);

  final MetronomeStateModel model;

  @override
  Widget build(BuildContext context) {
    const double height = 80;

    return SizedBox(
      height: height,
      width: double.infinity,
      child: Stack(
        children: [
          SizedBox(
            height: height,
            width: double.infinity,
            child: SceneBuilderWidget(
              builder: () => SceneController(
                config: SceneConfig(
                  autoUpdateRender: false,
                  painterWillChange: false,
                ),
                back: MetronomeSceneBack(
                  model,
                  brightness.value == Brightness.light
                      ? CupertinoColors.systemGrey5
                      : CupertinoColors.white,
                ),
              ),
            ),
          ),
          Center(
            child: Row(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                ElapsedBeatsVisualisation(model: model),
                SizedBox.fromSize(size: const Size.square(10)),
                ElapsedTimeVisualisation(model: model),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
