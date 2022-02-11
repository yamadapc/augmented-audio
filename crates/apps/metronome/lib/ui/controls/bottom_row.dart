import 'package:firebase_analytics/firebase_analytics.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/services.dart';
import 'package:flutter_mobx/flutter_mobx.dart';

import './bottom_row/tap_tempo_button.dart';
import '../../modules/state/metronome_state_controller.dart';

class PlayIntent extends Intent {}

class BottomRow extends StatelessWidget {
  final MetronomeStateController stateController;

  const BottomRow({Key? key, required this.stateController}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var model = stateController.model;
    return FocusableActionDetector(
      autofocus: true,
      shortcuts: {
        LogicalKeySet(LogicalKeyboardKey.space): PlayIntent(),
      },
      actions: {
        PlayIntent: CallbackAction<PlayIntent>(onInvoke: (_) {
          stateController.toggleIsPlaying();
        })
      },
      child: Row(
          mainAxisAlignment: MainAxisAlignment.start,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            Expanded(
              child: CupertinoButton(
                  color: CupertinoColors.activeBlue,
                  onPressed: () {
                    stateController.toggleIsPlaying();

                    var analytics = FirebaseAnalytics.instance;
                    analytics.logEvent(name: "BottomRow__toggleIsPlaying");
                  },
                  child: Observer(
                    builder: (_) => Text(model.isPlaying ? "Stop" : "Start",
                        style: const TextStyle(color: CupertinoColors.white)),
                  )),
            ),
            const SizedBox(width: 10),
            TapTempoButton(
                tapTempoController: stateController.tapTempoController,
                stateController: stateController)
          ]),
    );
  }
}
