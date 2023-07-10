import 'dart:io';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:flutter_platform_widgets/flutter_platform_widgets.dart';
import 'package:macos_ui/macos_ui.dart';
import 'package:metronome/bridge_generated.dart';
import 'package:metronome/modules/state/metronome_state_controller.dart';
import 'package:metronome/ui/constants.dart';

class SoundSelector extends StatelessWidget {
  final MetronomeStateController stateController;

  const SoundSelector({
    super.key,
    required this.stateController,
  });

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

  const SoundSelectorMobile({super.key, required this.stateController});

  @override
  Widget build(BuildContext context) {
    return Observer(
      builder: (context) {
        final selectedSoundName = sounds
            .firstWhere(
              (element) => element.tag == stateController.model.sound,
            )
            .name;
        return PlatformWidget(
          material: (_, __) => ButtonTheme(
            alignedDropdown: true,
            child: DropdownButton<MetronomeSound>(
              isExpanded: true,
              hint: const Text("Select sound"),
              value: sounds.firstWhere(
                (element) => element.tag == stateController.model.sound,
              ),
              // dropdownColor: CupertinoColors.activeBlue.withOpacity(0.8),
              items: sounds
                  .map(
                    (e) => DropdownMenuItem(
                      value: e,
                      child: Padding(
                        padding: buttonPadding,
                        child: Text(e.name),
                      ),
                    ),
                  )
                  .toList(),
              onChanged: (MetronomeSound? e) =>
                  stateController.setSound(e!.tag),
            ),
          ),
          cupertino: (_, __) => SizedBox(
            width: double.infinity,
            child: PlatformElevatedButton(
              color: CupertinoColors.activeBlue.withOpacity(0.8),
              padding: buttonPadding,
              child: Text(
                "Change sound ($selectedSoundName)",
                style: const TextStyle(color: CupertinoColors.white),
              ),
              onPressed: () {
                _showDialog(context);
              },
            ),
          ),
        );
      },
    );
  }

  void _showDialog(BuildContext context) {
    showPlatformModalSheet(
      context: context,
      builder: (ctx) => Container(
        color: CupertinoColors.secondarySystemBackground.resolveFrom(context),
        height: 300,
        child: Column(
          children: [
            Row(
              children: [
                const Spacer(),
                CupertinoButton(
                  child: const Text("Done"),
                  onPressed: () {
                    Navigator.of(context).pop();
                  },
                ),
              ],
            ),
            Container(
              height: 1,
              width: double.infinity,
              color: CupertinoColors.opaqueSeparator.resolveFrom(context),
            ),
            Expanded(
              child: CupertinoPicker(
                itemExtent: 40,
                onSelectedItemChanged: (index) {
                  stateController.setSound(sounds[index].tag);
                },
                scrollController: FixedExtentScrollController(
                  initialItem: sounds.indexWhere(
                    (element) => element.tag == stateController.model.sound,
                  ),
                ),
                children: sounds
                    .map(
                      (sound) => Container(
                        alignment: Alignment.center,
                        child: Text(sound.name),
                      ),
                    )
                    .toList(),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class SoundSelectorMacOS extends StatelessWidget {
  final MetronomeStateController stateController;

  const SoundSelectorMacOS({
    super.key,
    required this.stateController,
  });

  @override
  Widget build(BuildContext context) {
    return Observer(
      builder: (context) => SizedBox(
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
                  child: Padding(
                    padding: const EdgeInsets.only(bottom: 2.0),
                    child: Text(sound.name),
                  ),
                ),
              )
              .toList(),
        ),
      ),
    );
  }
}
