import 'package:flutter/cupertino.dart';
import 'package:flutter_mobx/flutter_mobx.dart';

import '../../modules/state/metronome_state_controller.dart';

class BottomRow extends StatelessWidget {
  final MetronomeStateController stateController;

  BottomRow({Key? key, required this.stateController}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var model = stateController.model;
    return Row(
        mainAxisAlignment: MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          Expanded(
            child: CupertinoButton(
                color: CupertinoColors.activeBlue,
                onPressed: () {
                  stateController.toggleIsPlaying();
                },
                child: Observer(
                  builder: (_) =>
                      Text(model.isPlaying ? "Stop" : "Start",
                          style: const TextStyle(color: CupertinoColors.white)),
                )),
          )
        ]);
  }
}
