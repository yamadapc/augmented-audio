import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/ui/common/styles.dart';
import 'package:flutter_daw_mock_ui/ui/common/tabs.dart';

import 'midi_editor/midi_editor.dart';

class BottomPanel extends StatelessWidget {
  const BottomPanel({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return DawTextStyle(
      child: Container(
        decoration: BoxDecoration(boxShadow: [
          BoxShadow(
            color: Colors.black.withOpacity(0.4),
            spreadRadius: 1.0,
            blurRadius: 5.0,
          )
        ], border: Border.all(color: const Color.fromRGBO(65, 65, 65, 1.0))),
        height: 400,
        child: PanelTabsView(tabs: [
          ConcretePanelTab(0, "MIDI Editor", const MIDIEditorView()),
          ConcretePanelTab(1, "FX", const TrackEffectsView()),
        ]),
      ),
    );
  }
}

class TrackEffectsView extends StatelessWidget {
  const TrackEffectsView({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Row(children: const [
      SizedBox(width: 300, child: AudioEffectView()),
      SizedBox(width: 300, child: AudioEffectView()),
      SizedBox(width: 300, child: AudioEffectView()),
    ]);
  }
}

class AudioEffectView extends StatelessWidget {
  const AudioEffectView({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
        margin: const EdgeInsets.all(8.0),
        decoration: BoxDecoration(
            color: const Color.fromRGBO(80, 80, 80, 1.0),
            boxShadow: [
              BoxShadow(
                color: Colors.black.withOpacity(0.4),
                spreadRadius: 1.0,
                blurRadius: 5.0,
              )
            ],
            border: Border.all(color: const Color.fromRGBO(65, 65, 65, 1.0))),
        child: Column(children: [
          Container(
              height: 20,
              padding: const EdgeInsets.only(left: 4, right: 4),
              width: double.infinity,
              decoration: const BoxDecoration(
                color: Color.fromRGBO(130, 130, 130, 1.0),
              ),
              child: const DefaultTextStyle(
                  style: TextStyle(color: Colors.black), child: Text("Delay"))),
          Expanded(
            child: Container(child: null),
          ),
        ]));
  }
}
