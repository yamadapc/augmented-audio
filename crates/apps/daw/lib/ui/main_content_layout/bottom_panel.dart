import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/state/ui_state.dart';
import 'package:flutter_daw_mock_ui/ui/common/styles.dart';
import 'package:flutter_daw_mock_ui/ui/common/tabs.dart';
import 'package:flutter_mobx/flutter_mobx.dart';

import 'bottom_panel/track_effects.dart';
import 'midi_editor/midi_editor.dart';
import 'midi_editor/midi_editor_view_model.dart';

class BottomPanel extends StatelessWidget {
  final DetailsPanelState detailsPanelState;

  const BottomPanel({Key? key, required this.detailsPanelState})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Observer(
      builder: (_) => DawTextStyle(
        child: Column(
          children: [
            GestureDetector(
                onPanUpdate: onPanUpdate,
                child: MouseRegion(
                  cursor: SystemMouseCursors.resizeUpDown,
                  child: Container(
                      decoration: const BoxDecoration(color: Colors.red),
                      height: 5,
                      width: double.infinity),
                )),
            Container(
              decoration: BoxDecoration(
                  boxShadow: [
                    BoxShadow(
                      color: Colors.black.withOpacity(0.4),
                      spreadRadius: 1.0,
                      blurRadius: 5.0,
                    )
                  ],
                  border:
                      Border.all(color: const Color.fromRGBO(65, 65, 65, 1.0))),
              height: detailsPanelState.height,
              child: PanelTabsView(
                  panelTabsState: detailsPanelState.panelTabsState,
                  tabs: [
                    ConcretePanelTab(
                        0,
                        "MIDI Editor",
                        MIDIEditorView(
                            model: MIDIEditorViewModel(
                                midiClipModel:
                                    detailsPanelState.midiClipModel))),
                    ConcretePanelTab(1, "FX", const TrackEffectsView()),
                  ]),
            )
          ],
        ),
      ),
    );
  }

  void onPanUpdate(DragUpdateDetails details) {
    detailsPanelState.updateHeight(details.delta.dy);
  }
}
