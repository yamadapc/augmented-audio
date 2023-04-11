import 'package:flutter/cupertino.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:flutter_platform_widgets/flutter_platform_widgets.dart';
import 'package:metronome/modules/state/metronome_state_controller.dart';
import 'package:metronome/modules/state/tap_tempo_controller.dart';
import 'package:metronome/ui/constants.dart';

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
      child: PlatformElevatedButton(
        color: CupertinoColors.activeBlue.withOpacity(0.8),
        padding: buttonPadding,
        onPressed: () {
          tapTempoController.onPress();
        },
        child: Observer(
          builder: (_) {
            var beatToDisplay = tapTempoController.presses.length %
                stateController.model.beatsPerBar;
            if (beatToDisplay == 0) {
              beatToDisplay = 4;
            }

            return Text(
              tapTempoController.presses.isNotEmpty ? "$beatToDisplay" : "Tap",
              style: const TextStyle(color: CupertinoColors.white),
            );
          },
        ),
      ),
    );
  }
}
