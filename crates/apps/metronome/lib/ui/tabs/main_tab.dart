import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:metronome/modules/context/app_context.dart';
import 'package:metronome/modules/state/metronome_state_controller.dart';
import 'package:metronome/ui/controls/beats_per_bar_control.dart';
import 'package:metronome/ui/controls/bottom_row.dart';
import 'package:metronome/ui/controls/tempo_control.dart';
import 'package:metronome/ui/controls/volume_control.dart';
import 'package:metronome/ui/tabs/main_tab/sound_selector.dart';
import 'package:metronome/ui/visualisation/visualisation.dart';

class PlayIntent extends Intent {}

class IncreaseTempoIntent extends Intent {
  final double value;

  const IncreaseTempoIntent({this.value = 1});
}

class DecreaseTempoIntent extends Intent {
  final double value;

  const DecreaseTempoIntent({this.value = 1});
}

class MainPageTab extends StatefulWidget {
  const MainPageTab({
    super.key,
    required this.stateController,
  });

  final MetronomeStateController stateController;

  @override
  State<MainPageTab> createState() => _MainPageTabState();
}

class _MainPageTabState extends State<MainPageTab> {
  ScrollController scrollController = ScrollController();

  @override
  void initState() {
    super.initState();
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
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

    final analytics = AppContext.of(context).analytics;
    analytics.logScreenView(
      screenClass: "MainPageTab",
      screenName: "Main Page",
    );
  }

  @override
  Widget build(BuildContext context) {
    return Shortcuts(
      shortcuts: {
        LogicalKeySet(LogicalKeyboardKey.space): PlayIntent(),
        LogicalKeySet(LogicalKeyboardKey.arrowLeft):
            const DecreaseTempoIntent(),
        LogicalKeySet(LogicalKeyboardKey.arrowRight):
            const IncreaseTempoIntent(),
        LogicalKeySet(LogicalKeyboardKey.shift, LogicalKeyboardKey.arrowLeft):
            const DecreaseTempoIntent(value: 10),
        LogicalKeySet(LogicalKeyboardKey.shift, LogicalKeyboardKey.arrowRight):
            const IncreaseTempoIntent(value: 10),
      },
      child: Actions(
        actions: {
          PlayIntent: CallbackAction<PlayIntent>(
            onInvoke: (_) {
              widget.stateController.toggleIsPlaying();
              return null;
            },
          ),
          DecreaseTempoIntent: CallbackAction<DecreaseTempoIntent>(
            onInvoke: (intent) {
              widget.stateController.decreaseTempo(decrement: intent.value);
              return null;
            },
          ),
          IncreaseTempoIntent: CallbackAction<IncreaseTempoIntent>(
            onInvoke: (intent) {
              widget.stateController.increaseTempo(increment: intent.value);
              return null;
            },
          ),
        },
        child: FocusScope(
          child: Focus(
            skipTraversal: true,
            autofocus: true,
            child: _buildContent(),
          ),
        ),
      ),
    );
  }

  Widget _buildContent() {
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
                      child: Column(
                        children: [
                          TempoControl(
                            stateController: widget.stateController,
                          ),
                          const Divider(thickness: 0.5),
                          VolumeControl(
                            stateController: widget.stateController,
                          ),
                        ],
                      ),
                    ),
                    const Spacer(),
                    SoundSelector(
                      stateController: widget.stateController,
                    ),
                    const SizedBox(height: 10),
                    BottomRow(stateController: widget.stateController),
                  ],
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
