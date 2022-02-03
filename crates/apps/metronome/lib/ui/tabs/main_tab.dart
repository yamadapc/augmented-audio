import 'package:firebase_analytics/firebase_analytics.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';

import '../../modules/state/metronome_state_controller.dart';
import '../controls/beats_per_bar_control.dart';
import '../controls/bottom_row.dart';
import '../controls/tempo_control.dart';
import '../controls/volume_control.dart';
import '../visualisation/visualisation.dart';

class MainPageTab extends StatefulWidget {
  const MainPageTab({
    Key? key,
    required this.stateController,
  }) : super(key: key);

  final MetronomeStateController stateController;

  @override
  State<MainPageTab> createState() => _MainPageTabState();
}

class _MainPageTabState extends State<MainPageTab> {
  @override
  void activate() {
    super.activate();
    var analytics = FirebaseAnalytics.instance;
    analytics.logScreenView(
        screenClass: "MainPageTab", screenName: "Main Page");
  }

  @override
  Widget build(BuildContext context) {
    return SafeArea(
      child: Padding(
        padding: const EdgeInsets.all(8.0),
        child: Column(
          children: [
            Visualisation(model: widget.stateController.model),
            BeatsPerBarControl(stateController: widget.stateController),
            const SizedBox(height: 30),
            Expanded(
              child: Center(
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: <Widget>[
                    TempoControl(stateController: widget.stateController),
                    VolumeControl(stateController: widget.stateController),
                    const Spacer(),
                    BottomRow(stateController: widget.stateController)
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
