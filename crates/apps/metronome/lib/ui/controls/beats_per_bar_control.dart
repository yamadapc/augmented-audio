import 'package:flutter/cupertino.dart';
import 'package:flutter_mobx/flutter_mobx.dart';

import '../../modules/state/metronome_state_controller.dart';

class BeatsPerBarControl extends StatelessWidget {
  const BeatsPerBarControl({
    Key? key,
    required this.stateController,
  }) : super(key: key);

  final MetronomeStateController stateController;

  @override
  Widget build(BuildContext context) {
    return Observer(
      builder: (_) => CupertinoSegmentedControl(
          groupValue: stateController.model.beatsPerBar,
          children: {
            1: segmentedControlText("None"),
            2: segmentedControlText("2/4"),
            3: segmentedControlText("3/4"),
            4: segmentedControlText("4/4"),
            5: segmentedControlText("5/4"),
            6: segmentedControlText("6/4"),
            7: segmentedControlText("7/4"),
          },
          padding: const EdgeInsets.all(0.0),
          onValueChanged: (int value) {
            stateController.setBeatsPerBar(value);
          }),
    );
  }
}

Widget segmentedControlText(String s) {
  return Padding(
    padding: const EdgeInsets.only(left: 3.0, right: 3.0),
    child: Text(s, textScaleFactor: 0.7),
  );
}
