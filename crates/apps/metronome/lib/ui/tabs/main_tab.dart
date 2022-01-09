import 'package:flutter/cupertino.dart';
import 'package:metronome/modules/state/metronome_state_controller.dart';

import '../controls/bottom_row.dart';
import '../controls/tempo_control.dart';
import '../controls/volume_control.dart';
import '../visualisation/visualisation.dart';

class MainPageTab extends StatelessWidget {
  const MainPageTab({
    Key? key,
    required this.stateController,
  }) : super(key: key);

  final MetronomeStateController stateController;

  @override
  Widget build(BuildContext context) {
    return SafeArea(
      child: Padding(
        padding: const EdgeInsets.all(8.0),
        child: Column(
          children: [
            Visualisation(model: stateController.model),
            Expanded(
              child: Center(
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: <Widget>[
                    TempoControl(stateController: stateController),
                    VolumeControl(stateController: stateController),
                    const Spacer(),
                    BottomRow(stateController: stateController)
                  ],
                ),
              ),
            )
          ],
        ),
      ),
    );
  }
}