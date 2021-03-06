import 'package:flutter/cupertino.dart';
import 'package:flutter_mobx/flutter_mobx.dart';

import '../../../modules/state/metronome_state_controller.dart';
import '../../../modules/state/tap_tempo_controller.dart';

class TapTempoButton extends StatelessWidget {
  const TapTempoButton({
    Key? key,
    required this.tapTempoController,
    required this.stateController,
  }) : super(key: key);

  final TapTempoController tapTempoController;
  final MetronomeStateController stateController;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 70,
      child: CupertinoButton(
        color: CupertinoColors.activeBlue.withOpacity(0.8),
        padding: const EdgeInsets.all(14),
        onPressed: () {
          tapTempoController.onPress();
        },
        child: Observer(builder: (_) {
          var beatToDisplay = tapTempoController.presses.length %
              stateController.model.beatsPerBar;
          if (beatToDisplay == 0) {
            beatToDisplay = 4;
          }

          return Text(
              tapTempoController.presses.isNotEmpty ? "$beatToDisplay" : "Tap",
              style: const TextStyle(color: CupertinoColors.white));
        }),
      ),
    );
  }
}
