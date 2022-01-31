import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:graphx/graphx.dart';
import 'package:metronome/modules/state/metronome_state_model.dart';

import 'scene.dart';

class Visualisation extends StatelessWidget {
  const Visualisation({
    Key? key,
    required this.model,
  }) : super(key: key);

  final MetronomeStateModel model;

  @override
  Widget build(BuildContext context) {
    double height = 80;

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
                back: MetronomeSceneBack(model),
              ),
              child: null,
            ),
          ),
          Center(
            child: Container(
              padding: const EdgeInsets.only(left: 15, right: 15),
              decoration: BoxDecoration(
                  border:
                      Border.all(color: CupertinoColors.secondarySystemFill),
                  borderRadius: BorderRadius.circular(2.0),
                  color: CupertinoColors.secondarySystemBackground
                      .withOpacity(0.7)),
              child: Observer(builder: (_) {
                if (!model.isPlaying) {
                  return const Text("0/0");
                }

                var beat = Math.floor((model.playhead) % 4)! + 1;
                var bar = Math.floor(model.playhead / 4)!;

                return Text(
                    "${beat.toStringAsFixed(0)}/${bar.toStringAsFixed(0)}");
              }),
            ),
          ),
        ],
      ),
    );
  }
}
