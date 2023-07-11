import 'package:flutter/cupertino.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:flutter_platform_widgets/flutter_platform_widgets.dart';
import 'package:metronome/modules/context/app_context.dart';
import 'package:metronome/modules/state/metronome_state_controller.dart';
import 'package:metronome/ui/constants.dart';
import 'package:metronome/ui/controls/bottom_row/tap_tempo_button.dart';

class BottomRow extends StatelessWidget {
  final MetronomeStateController stateController;

  const BottomRow({super.key, required this.stateController});

  @override
  Widget build(BuildContext context) {
    final model = stateController.model;
    return Row(
      children: [
        Expanded(
          child: PlatformElevatedButton(
            key: const Key("PlayButton"),
            padding: buttonPadding,
            color: CupertinoColors.activeBlue,
            onPressed: () {
              stateController.toggleIsPlaying();

              final analytics = AppContext.of(context).analytics;
              analytics.logEvent(name: "BottomRow__toggleIsPlaying");
            },
            child: Observer(
              builder: (_) => Text(
                model.isPlaying ? "Stop" : "Start",
                style: const TextStyle(color: CupertinoColors.white),
              ),
            ),
          ),
        ),
        const SizedBox(width: 10),
        TapTempoButton(
          tapTempoController: stateController.tapTempoController,
          stateController: stateController,
        )
      ],
    );
  }
}
