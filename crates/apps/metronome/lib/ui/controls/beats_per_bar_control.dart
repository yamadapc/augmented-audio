import 'dart:io';

import 'package:flutter/cupertino.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:metronome/modules/context/app_context.dart';
import 'package:metronome/modules/state/metronome_state_controller.dart';
import 'package:metronome/ui/controls/platform/platform_segmented_control.dart';

class BeatsPerBarControl extends StatelessWidget {
  const BeatsPerBarControl({
    super.key,
    required this.stateController,
  });

  final MetronomeStateController stateController;

  @override
  Widget build(BuildContext context) {
    return Observer(
      builder: (_) => PlatformSegmentedControl(
        value: stateController.model.beatsPerBar,
        options: const [1, 2, 3, 4, 5, 6, 7],
        optionLabelBuilder: (int value) {
          return value == 1 ? "None" : "$value/4";
        },
        onValueChanged: (int value) {
          stateController.setBeatsPerBar(value);
          final analytics = AppContext.of(context).analytics;
          analytics.logEvent(name: "BeatsPerBarControl__onValueChanged");
        },
      ),
    );
  }
}

Widget segmentedControlText(String s) {
  return Padding(
    padding: const EdgeInsets.only(left: 3.0, right: 3.0),
    child: Text(s, textScaleFactor: Platform.isMacOS ? 0.85 : 1.0),
  );
}
