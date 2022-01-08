import 'package:flutter/cupertino.dart';
import 'package:graphx/graphx.dart';
import 'package:mobx/mobx.dart';

import 'scene.dart';

class Visualisation extends StatelessWidget {
  const Visualisation({
    Key? key,
    required this.playhead,
  }) : super(key: key);

  final Observable<double> playhead;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 120,
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
    );
  }
}
