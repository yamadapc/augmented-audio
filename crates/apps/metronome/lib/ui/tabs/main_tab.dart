import 'dart:io';

import 'package:firebase_analytics/firebase_analytics.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:metronome/bridge_generated.dart';

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
  ScrollController scrollController = ScrollController();

  @override
  void initState() {
    super.initState();
    fireViewedAnalytics();
  }

  @override
  void activate() {
    super.activate();
    fireViewedAnalytics();
  }

  void fireViewedAnalytics() {
    if (Platform.environment.containsKey('FLUTTER_TEST')) {
      return;
    }

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
                    SingleChildScrollView(
                        controller: scrollController,
                        child: Column(children: [
                          TempoControl(stateController: widget.stateController),
                          const Divider(thickness: 0.5),
                          VolumeControl(
                              stateController: widget.stateController),
                        ])),
                    const Spacer(),
                    Padding(
                      padding: const EdgeInsets.fromLTRB(15.0, 0.0, 10.0, 10.0),
                      child: SizedBox(
                        width: double.infinity,
                        height: 25,
                        child: MacosPopupButton<MetronomeSoundTypeTag>(
                          value: widget.stateController.model.sound,
                          focusNode: FocusNode(skipTraversal: true),
                          onChanged: (item) {
                            widget.stateController.setSound(item!);
                          },
                          popupColor: CupertinoColors.activeBlue,
                          items: const [
                            MacosPopupMenuItem(
                                value: MetronomeSoundTypeTag.Sine,
                                child: Text("Sine")),
                            MacosPopupMenuItem(
                                value: MetronomeSoundTypeTag.Tube,
                                child: Text("Tube"))
                          ],
                        ),
                      ),
                    ),
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
