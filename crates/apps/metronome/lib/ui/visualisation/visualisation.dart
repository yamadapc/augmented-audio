import 'package:flutter/cupertino.dart';
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
    return SizedBox(
      height: 120,
      width: double.infinity,
      child: SceneBuilderWidget(
        builder: () =>
            SceneController(
              config: SceneConfig(
                autoUpdateRender: false,
                painterWillChange: false,
              ),
              back: MetronomeSceneBack(model),
        ),
        child: null,
      ),
    );
  }
}
