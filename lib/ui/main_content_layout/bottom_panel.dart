import 'package:flutter/material.dart';
import 'package:flutter_daw_mock_ui/ui/common/styles.dart';
import 'package:flutter_daw_mock_ui/ui/common/tabs.dart';
import 'package:flutter_mobx/flutter_mobx.dart';
import 'package:mobx/mobx.dart';

import 'bottom_panel/track_effects.dart';
import 'midi_editor/midi_editor.dart';

Observable<double> heightObservable = Observable(200);

class BottomPanel extends StatelessWidget {
  const BottomPanel({Key? key}) : super(key: key);

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
              height: heightObservable.value,
              child: PanelTabsView(tabs: [
                ConcretePanelTab(0, "MIDI Editor", MIDIEditorView()),
                ConcretePanelTab(1, "FX", const TrackEffectsView()),
              ]),
            )
          ],
        ),
      ),
    );
  }

  void onPanUpdate(DragUpdateDetails details) {
    runInAction(() {
      heightObservable.value -= details.delta.dy;
    });
  }
}
