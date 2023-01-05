import 'dart:io';

import 'package:flutter/cupertino.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:metronome/bridge_generated.dart';
import 'package:metronome/modules/state/metronome_state_controller.dart';

class SoundSelector extends StatelessWidget {
  final MetronomeStateController stateController;

  const SoundSelector({
    Key? key,
    required this.stateController,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    // detect if ios flutter
    if (Platform.isMacOS) {
      return SoundSelectorMacOS(stateController: stateController);
    } else {
      return SoundSelectorMobile(stateController: stateController);
    }
  }
}

class MetronomeSound {
  final String name;
  final MetronomeSoundTypeTag tag;

  const MetronomeSound({required this.name, required this.tag});
}

const sounds = [
  MetronomeSound(name: "Sine", tag: MetronomeSoundTypeTag.Sine),
  MetronomeSound(name: "Tube", tag: MetronomeSoundTypeTag.Tube),
  MetronomeSound(name: "Snap", tag: MetronomeSoundTypeTag.Snap),
  MetronomeSound(name: "Glass", tag: MetronomeSoundTypeTag.Glass),
];

class SoundSelectorMobile extends StatelessWidget {
  final MetronomeStateController stateController;

  const SoundSelectorMobile({Key? key, required this.stateController})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Observer(
      builder: (_) {
        final selectedSoundName = sounds
            .firstWhere(
              (element) => element.tag == stateController.model.sound,
            )
            .name;
        return CupertinoButton(
          color: CupertinoColors.activeBlue.withOpacity(0.8),
          child: Text(
            "Change sound ($selectedSoundName)",
          ),
          onPressed: () {
            _showDialog(context);
          },
        );
      },
    );
  }

  void _showDialog(BuildContext context) {
    showCupertinoModalPopup(
      context: context,
      builder: (ctx) => Container(
        color: CupertinoColors.systemBackground.resolveFrom(context),
        height: 216,
        child: SafeArea(
          child: CupertinoPicker(
            itemExtent: 32,
            onSelectedItemChanged: (index) {
              stateController.setSound(sounds[index].tag);
            },
            scrollController: FixedExtentScrollController(
              initialItem: sounds.indexWhere(
                (element) => element.tag == stateController.model.sound,
              ),
            ),
            children: sounds.map((sound) => Text(sound.name)).toList(),
          ),
        ),
      ),
    );
  }
}

class SoundSelectorMacOS extends StatelessWidget {
  final MetronomeStateController stateController;

  const SoundSelectorMacOS({
    Key? key,
    required this.stateController,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Observer(
      builder: (ctx) => SizedBox(
        width: double.infinity,
        height: 25,
        child: MacosPopupButton<MetronomeSoundTypeTag>(
          value: stateController.model.sound,
          focusNode: FocusNode(skipTraversal: true),
          onChanged: (item) {
            stateController.setSound(item!);
          },
          popupColor: CupertinoColors.activeBlue,
          items: sounds
              .map(
                (sound) => MacosPopupMenuItem(
                  value: sound.tag,
                  child: Text(sound.name),
                ),
              )
              .toList(),
        ),
      ),
    );
  }
}
